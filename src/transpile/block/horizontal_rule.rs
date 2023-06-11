use crate::transpile::html::{create, Html};

pub fn from(input: &str) -> Option<Html> {
    match input.trim() {
        "***" | "---" | "___" => Some(create("hr", None, None)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_from() {
        let markdown = " ";
        assert_eq!(from(markdown), None);
        let markdown = "**";
        assert_eq!(from(markdown), None);
        let markdown = "** *";
        assert_eq!(from(markdown), None);
        let markdown = "**foo**";
        assert_eq!(from(markdown), None);
    }

    #[test]
    fn some_from_three_asterisk() {
        let markdown = "***";
        let html = Some("<hr>".to_string());
        assert_eq!(from(markdown), html);
    }

    #[test]
    fn some_from_three_hyphen() {
        let markdown = "---";
        let html = Some("<hr>".to_string());
        assert_eq!(from(markdown), html);
    }

    #[test]
    fn some_from_three_underscore() {
        let markdown = "___";
        let html = Some("<hr>".to_string());
        assert_eq!(from(markdown), html);
    }

    // While offically supported, I want to use a sticter subset of valid md.
    // Therefore any more than 3 chars is not treated as an hr.
    #[test]
    fn none_from_many_asterisk() {
        let markdown = "****";
        assert_eq!(from(markdown), None);
    }

    #[test]
    fn none_from_many_hyphen() {
        let markdown = "----";
        assert_eq!(from(markdown), None);
    }

    #[test]
    fn none_from_many_underscore() {
        let markdown = "____";
        assert_eq!(from(markdown), None);
    }
}
