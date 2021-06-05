use pulldown_cmark::{html, Options, Parser};

fn main() {
    let input = r#"- [ ] Item 1
- [ ] Item 2
- [ ] Item 3

  More item 3"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(input, options);

    for event in parser {
        dbg!(event);
    }
}
