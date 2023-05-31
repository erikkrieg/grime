use std::fs;

use grime::transpile::*;

fn read_fixture(file: &str) -> String {
    fs::read_to_string(format!("./tests/paragraph_fixtures/{file}"))
        .unwrap_or_else(|_| panic!("{file} not found"))
}

#[test]
fn headings_integration_test() {
    let markdown = read_fixture("paragraphs.md");
    let html = read_fixture("paragraphs.html");
    assert_eq!(parse(&markdown), html.trim_end());
}
