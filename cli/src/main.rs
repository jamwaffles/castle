use castle_core::{
    dao::note::{Note, TextNoteCreated},
    storage::FileStorage,
};
use event_sauce::{CreatePersister, Persistable};
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "castle", about = "Keep ur notes.")]
enum Command {
    /// Create a new note.
    Add { title: String },

    /// List notes.
    List,
}

fn main() {
    let command = Command::from_args();

    let mut storage = FileStorage::open("./test-db.json").unwrap();

    match command {
        Command::Add { title } => {
            println!("Title: {}", title);

            // Collect multiline body
            let body = {
                let stdin = io::stdin();
                let mut lines = stdin.lock().lines();

                let mut collected = Vec::new();

                println!("Write some stuff, press Ctrl + D when done");

                while let Some(line) = lines.next() {
                    collected.push(line.expect("No line"));
                }

                collected.join("\n")
            };

            let note = Note::create(TextNoteCreated { title, body })
                .persist(&mut storage)
                .unwrap();

            dbg!(note);

            println!("Note saved");
        }
        Command::List => {
            println!("{} notes\n", storage.items.notes.len());

            println!(
                "{}",
                storage
                    .items
                    .notes
                    .iter()
                    .map(|note| format!("# {}\n\n{}", note.title, note.body))
                    .collect::<Vec<_>>()
                    .join("\n\n---\n\n")
            );
        }
    }
}
