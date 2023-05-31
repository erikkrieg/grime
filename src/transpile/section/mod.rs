use super::{html::Html, ParserRules};

mod blockquote;
mod code;

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
    next_line: Option<&&str>,
) -> (Html, Option<Html>, Option<Html>) {
    let mut html_out = (line.trim().to_string(), None, None);
    if rules.section.blockquote {
        html_out = blockquote::replace(&html_out.0, rules, next_line);
    }
    if rules.section.code {
        // println!("todo: implement code section");
    }
    html_out
}
