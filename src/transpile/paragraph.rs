use crate::transpile::html::{create, Html};

pub fn from(input: &str) -> Option<Html> {
    let input = input.trim();
    if input.is_empty() {
        None
    } else {
        Some(create("p", Some(input), None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_paragraph_from() {
        let markdown = " ";
        assert_eq!(from(markdown), None);
    }
    #[test]
    fn paragraph_from() {
        let markdown = "This is a single line paragraph.";
        let html = Some("<p>This is a single line paragraph.</p>".to_string());
        assert_eq!(from(markdown), html);
    }
}
