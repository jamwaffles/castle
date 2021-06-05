fn main() {
    println!("Diff test");

    let left = r#"line 1

line 2

line 3

deleted 4"#;

    let right = r#"line 1

edited 2

line 3

added 1"#;

    let diff = diff::chars(&left, &right);

    dbg!(diff);
}
