use super::{html::Html, ParserRules};
use std::iter::Peekable;

pub mod blockquote;
pub mod code;

#[derive(Debug)]
pub struct SectionRules {
    pub blockquote: bool,
    pub code: bool,
}

impl Default for SectionRules {
    fn default() -> Self {
        Self {
            blockquote: true,
            code: true,
        }
    }
}

pub fn replace(
    line: &str,
    rules: &mut ParserRules,
    lines: &mut Peekable<impl Iterator<Item = &str>>,
) -> (Html, Option<Html>, Option<Html>) {
    let mut html_out = (line.trim().to_string(), None, None);
    if rules.section.blockquote {
        html_out = blockquote::replace(&html_out.0, rules, lines.peek());
    }
    if rules.section.code {
        html_out.0 = code::replace(&html_out.0, rules, lines);
    }
    html_out
}
