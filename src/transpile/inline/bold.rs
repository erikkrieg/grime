use crate::transpile::html::Html;
use regex::Regex;

pub fn replace(text: &str) -> Html {
    let pattern = Regex::new(r"(\*\*|__)(.*?)(\*\*|__)").expect("Bold replace regex was invalid");
    pattern.replace_all(text, "<strong>$2</strong>").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_replace_one_word() {
        let markdown = "Contains a **bold** word.";
        let html = "Contains a <strong>bold</strong> word.";
        assert_eq!(replace(markdown), html);
    }

    #[test]
    fn bold_replace_multiple_words() {
        let markdown = "Contains a **couple bold** words.";
        let html = "Contains a <strong>couple bold</strong> words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_multiple_tags() {
        let markdown = "**Contains** a **few bold** words.";
        let html = "<strong>Contains</strong> a <strong>few bold</strong> words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_alt_syntax() {
        let markdown = "Using __underscores__ to bold.";
        let html = "Using <strong>underscores</strong> to bold.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_no_match() {
        let markdown = "No bold words.";
        let html = "No bold words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_no_match_partial() {
        let markdown = "No **bold words.";
        let html = "No **bold words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_no_match_partial_alt_syntax() {
        let markdown = "No bold__ words.";
        let html = "No bold__ words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_inside_word() {
        let markdown = "Em**bolden**ed";
        let html = "Em<strong>bolden</strong>ed";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn bold_replace_inside_word_alt_syntax() {
        let markdown = "Em__bolden__ed";
        let html = "Em<strong>bolden</strong>ed";
        assert_eq!(replace(markdown), html)
    }
}
