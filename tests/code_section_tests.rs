use grime::transpile::*;
use pretty_assertions::assert_eq;
use std::fs;

fn read_fixture(file: &str) -> String {
    fs::read_to_string(format!("./tests/code_section_fixtures/{file}"))
        .unwrap_or_else(|_| panic!("{file} not found"))
}

#[test]
fn code_section_integration_test() {
    let markdown = read_fixture("code.md");
    let html = read_fixture("code.html");
    assert_eq!(parse(&markdown), html.trim_end());
}
