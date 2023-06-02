use super::{html::Html, ParserRules};
use regex::Regex;

#[derive(Debug)]
pub struct InlineRules {
    pub bold: bool,
    pub code: bool,
    pub escape: bool,
    pub italic: bool,
}

impl Default for InlineRules {
    fn default() -> Self {
        Self {
            bold: true,
            code: true,
            escape: true,
            italic: true,
        }
    }
}

// This is a lazy fix for a bug that occurs due to my lazy regex implementation
// of inline elements. In the case of ***foo*** or ___foo___ the <strong> and <em>
// tags will now wrap correctly.
// I plan to refactor out the use of regex in favor of a custom parser in the
// future. I'm fine with this gnarly fix in the meantime.
pub fn hacky_fix(text: &str) -> Html {
    let pattern =
        Regex::new(r"(<strong><em>)(.*?)(</strong></em>)").expect("Hacky fix regex was invalid");
    pattern
        .replace_all(text, "<strong><em>$2</em></strong>")
        .to_string()
}

pub fn replace(line: &str, _rules: &mut ParserRules) -> Html {
    let mut html = Html::new();
    let mut chars = line.chars().peekable();
    let mut open_bold = false;
    let mut open_italic = false;
    let mut open_code = false;
    while let Some(ch) = chars.next() {
        match (ch, chars.peek(), open_code) {
            ('*', Some('*'), false) | ('_', Some('_'), false) => {
                let tag = if !open_bold { "<strong>" } else { "</strong>" };
                html.push_str(tag);
                chars.next();
                open_bold = !open_bold;
            }
            ('*' | '_', _, false) => {
                let tag = if !open_italic { "<em>" } else { "</em>" };
                html.push_str(tag);
                open_italic = !open_italic;
            }
            ('`', _, _) => {
                let tag = if !open_code { "<code>" } else { "</code>" };
                html.push_str(tag);
                open_code = !open_code;
            }
            ('\\', Some(_), false) => html.push(chars.next().unwrap()),
            _ => html.push(ch),
        };
    }
    hacky_fix(&html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_replace_test() {
        let mut rules = ParserRules::default();
        let markdown = "**bold** __underscore bold__ and `code` and _italic_ *asterisk italic*.";
        let html = "<strong>bold</strong> <strong>underscore bold</strong> and <code>code</code> and <em>italic</em> <em>asterisk italic</em>.";
        assert_eq!(replace(markdown, &mut rules), html);
    }

    #[test]
    fn bad_underscore_and_asterisk_replace_test() {
        let mut rules = ParserRules::default();
        let markdown = "_*bad*_ ***foo_** *bar_";
        // I'm actually not yet sure what the correct output is here. I can make
        // some assumption or be strict and panic. TBD.
        let html = "<em></em>bad<em></em> <strong><em>foo</em></strong> <em>bar</em>";
        assert_eq!(replace(markdown, &mut rules), html);
    }

    #[test]
    fn bold_and_italic_replace_test() {
        let mut rules = ParserRules::default();
        let markdown = "***one*** ___two___ _**three**_";
        let html = "<strong><em>one</em></strong> <strong><em>two</em></strong> <em><strong>three</strong></em>";
        assert_eq!(replace(markdown, &mut rules), html);
    }

    #[test]
    fn code_replace_test() {
        let mut rules = ParserRules::default();
        let markdown = "`_not italic_ ******`";
        let html = "<code>_not italic_ ******</code>";
        assert_eq!(replace(markdown, &mut rules), html);
    }

    #[test]
    fn escape_replace_test() {
        let mut rules = ParserRules::default();
        let markdown = r"\_not italic\_";
        let html = "_not italic_";
        assert_eq!(replace(markdown, &mut rules), html);
    }
}
