use super::{html::Html, ParserRules};

pub mod heading;
pub mod horizontal_rule;
pub mod paragraph;

#[derive(Debug)]
pub struct BlockRules {
    pub heading: bool,
    pub paragraph: bool,
    pub horizontal_rule: bool,
}

impl Default for BlockRules {
    fn default() -> Self {
        Self {
            heading: true,
            paragraph: true,
            horizontal_rule: true,
        }
    }
}

pub fn replace(line: &str, rules: &mut ParserRules) -> Html {
    let mut line = line.trim().to_string();
    if  rules.block.horizontal_rule && let Some(hr) = horizontal_rule::from(&line) {
        line = hr;
    } else if rules.block.heading && let Some(h) = heading::from(&line) {
       line = h;
    }  else if rules.block.paragraph && let Some(p) = paragraph::from(&line) {
        line = p;
    }
    line
}
