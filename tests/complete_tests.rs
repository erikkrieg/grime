use grime::transpile::*;
use pretty_assertions::assert_eq;
use std::fs;

fn read_fixture(file: &str) -> String {
    fs::read_to_string(format!("./tests/complete_fixtures/{file}"))
        .unwrap_or_else(|_| panic!("{file} not found"))
}

#[test]
fn complete_integration_test() {
    let markdown = read_fixture("complete.md");
    let html = read_fixture("complete.html");
    assert_eq!(parse(&markdown), html.trim_end());
}

#[test]
fn complete_parser_integration_test() {
    let markdown = read_fixture("complete_parse.md");
    let html = read_fixture("complete_parse.html");
    assert_eq!(parse(&markdown), html);
}
