use std::fs;

use grime::transpile::*;

#[test]
fn headings_integration_test() {
    let markdown = fs::read_to_string("./tests/headings_fixtures/headings.md")
        .expect("tests/headings.md not found");
    let html = fs::read_to_string("./tests/headings_fixtures/headings.html")
        .expect("tests/headings.html not found");
    assert_eq!(transpile(&markdown), html.trim_end());
}
