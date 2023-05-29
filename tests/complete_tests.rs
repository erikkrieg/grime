use grime::transpile::*;
use std::fs;

fn read_fixture(file: &str) -> String {
    fs::read_to_string(format!("./tests/complete_fixtures/{file}"))
        .unwrap_or_else(|_| panic!("{file} not found"))
}

#[test]
fn headings_integration_test() {
    let markdown = read_fixture("complete.md");
    let html = read_fixture("complete.html");
    assert_eq!(transpile(&markdown), html.trim_end());
}
