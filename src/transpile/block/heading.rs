use crate::transpile::html::{create, Html};
use std::char;

pub fn from(input: &str) -> Option<Html> {
    let index = input.find(char::is_whitespace).unwrap_or(0);
    let first = &input[0..index];
    let inner = &input[(index + 1)..];
    let tag_name = match first {
        "#" => Some("h1"),
        "##" => Some("h2"),
        "###" => Some("h3"),
        "####" => Some("h4"),
        "#####" => Some("h5"),
        "######" => Some("h6"),
        _ => None,
    };
    tag_name.map(|t| create(t, Some(inner), None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_from() {
        let headings = ["#", "##", "###", "####", "#####", "######"];
        headings.iter().for_each(|h| {
            let level = h.len();
            let heading_md = format!("{h} Heading {level}");
            assert_eq!(
                from(&heading_md).expect(&format!("Expected to create heading from {heading_md}")),
                format!("<h{level}>Heading {level}</h{level}>")
            )
        });
    }

    #[test]
    fn invalid_heading_from() {
        let invalid = "####### Heading 7";
        assert_eq!(from(invalid), None);
    }
}
