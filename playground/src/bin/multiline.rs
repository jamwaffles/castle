use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut collected = Vec::new();

    println!("Write some stuff, press Ctrl + D when done");

    while let Some(line) = lines.next() {
        collected.push(line.expect("No line"));
    }

    println!("Lines accepted:\n\n{}", collected.join("\n"));

    Ok(())
}
