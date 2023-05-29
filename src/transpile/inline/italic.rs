use crate::transpile::html::Html;
use regex::Regex;

pub fn replace(text: &str) -> Html {
    let pattern = Regex::new(r"(\*|_)(.*?)(\*|_)").expect("Italic replace regex was invalid");
    pattern.replace_all(text, "<em>$2</em>").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn italic_replace_one_word() {
        let markdown = "Contains a *italic* word.";
        let html = "Contains a <em>italic</em> word.";
        assert_eq!(replace(markdown), html);
    }

    #[test]
    fn italic_replace_multiple_words() {
        let markdown = "Contains a *couple italic* words.";
        let html = "Contains a <em>couple italic</em> words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_multiple_tags() {
        let markdown = "*Contains* a *few italic* words.";
        let html = "<em>Contains</em> a <em>few italic</em> words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_alt_syntax() {
        let markdown = "Using _underscores_ to italic.";
        let html = "Using <em>underscores</em> to italic.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_no_match() {
        let markdown = "No italic words.";
        let html = "No italic words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_no_match_partial() {
        let markdown = "No *italic words.";
        let html = "No *italic words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_no_match_partial_alt_syntax() {
        let markdown = "No italic_ words.";
        let html = "No italic_ words.";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_inside_word() {
        let markdown = "Em*italicen*ed";
        let html = "Em<em>italicen</em>ed";
        assert_eq!(replace(markdown), html)
    }

    #[test]
    fn italic_replace_inside_word_alt_syntax() {
        let markdown = "Italiz_ed_";
        let html = "Italiz<em>ed</em>";
        assert_eq!(replace(markdown), html)
    }
}
