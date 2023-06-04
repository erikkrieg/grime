use super::{html::Html, ParserRules};

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

pub fn replace(line: &str, _rules: &mut ParserRules) -> Result<Html, &'static str> {
    let mut html = Html::new();
    let mut chars = line.chars().peekable();
    let mut open_bold = false;
    let mut open_italic_char = None;
    let mut prev: Option<char> = None;
    while let Some(ch) = chars.next() {
        match (prev, ch, chars.peek(), open_italic_char) {
            (_, '*', Some('*'), Some('_') | None) => {
                let tag = if !open_bold { "<strong>" } else { "</strong>" };
                chars.next();
                if !open_bold && chars.peek().is_none() {
                    html.push_str("**");
                } else {
                    html.push_str(tag);
                    open_bold = !open_bold;
                }
            }
            (prev, '_', Some(next), None) => {
                if prev.unwrap_or(' ').is_whitespace() {
                    open_italic_char = Some('_');
                    html.push_str("<em>");
                    if next == &'_' {
                        let escaped = &chars.by_ref().take_while(|c| c == &'_').collect::<String>();
                        html.push_str(escaped);
                    }
                } else {
                    html.push('_');
                }
            }
            (_, '_', next, Some('_')) => {
                if next.unwrap_or(&' ').is_whitespace() {
                    open_italic_char = None;
                    html.push_str("</em>");
                } else {
                    html.push('_');
                }
            }
            (_, '*', Some(_), None) => {
                open_italic_char = Some('*');
                html.push_str("<em>");
            }
            (_, '*', _, Some('*')) => {
                open_italic_char = None;
                html.push_str("</em>");
            }
            (_, '`', _, _) => {
                html.push_str("<code>");
                let mut is_code_closed = false;
                html.push_str(
                    &chars
                        .by_ref()
                        .take_while(|c| {
                            is_code_closed = c == &'`';
                            !is_code_closed
                        })
                        .collect::<String>(),
                );
                if !is_code_closed {
                    return Err("Inline code element is missing closing backtick");
                }
                html.push_str("</code>");
            }
            (_, '\\', Some(_), _) => html.push(chars.next().unwrap()),
            _ => html.push(ch),
        };
        prev = Some(ch);
    }
    //Ok(hacky_fix(&html))
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn italic_replace() {
        let mut rules = ParserRules::default();
        let markdown = "_foo_ *bar* foo*bar*baz foo_bar_baz end_";
        let html = "<em>foo</em> <em>bar</em> foo<em>bar</em>baz foo_bar_baz end_";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn italic_mixed_syntax_replace() {
        let mut rules = ParserRules::default();
        let markdown = "_foo*bar_";
        let html = "<em>foo*bar</em>";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn italic_edge_case_replace() {
        let mut rules = ParserRules::default();
        let markdown = "_foo_.";
        let html = "<em>foo_.";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn bold_replace() {
        let mut rules = ParserRules::default();
        let markdown = "**foo** foo**bar**baz foo__bar__baz end**";
        let html = "<strong>foo</strong> foo<strong>bar</strong>baz foo__bar__baz end**";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn bold_and_italic_replace() {
        let mut rules = ParserRules::default();
        let markdown = "***foo*** _**bar**_ foo***bar***baz";
        let html = "<strong><em>foo</em></strong> <em><strong>bar</strong></em> foo<strong><em>bar</em></strong>baz";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn awkward_bold_and_italic_replace() {
        let mut rules = ParserRules::default();
        let markdown = "**_???_**";
        let html = "<strong>_???_</strong>";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn code_replace() {
        let mut rules = ParserRules::default();
        let markdown = "`foo` and `bar`";
        let html = "<code>foo</code> and <code>bar</code>";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn code_escape_replace() {
        let mut rules = ParserRules::default();
        let markdown = "Examples: `**bold** and _italic_`";
        let html = "Examples: <code>**bold** and _italic_</code>";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    #[test]
    fn code_no_closing_replace() {
        let mut rules = ParserRules::default();
        let markdown = "`foo";
        assert_eq!(
            replace(markdown, &mut rules).unwrap_err(),
            "Inline code element is missing closing backtick"
        );
    }

    #[test]
    fn escape_replace_test() {
        let mut rules = ParserRules::default();
        let markdown = r"*italic* and \*not italic\*";
        let html = "<em>italic</em> and *not italic*";
        assert_eq!(replace(markdown, &mut rules).unwrap(), html);
    }

    // TODO: panic if line ends with unclosed elements
}
