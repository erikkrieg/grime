use super::{html::Html, ParserRules};

pub mod heading;
pub mod paragraph;

#[derive(Debug)]
pub struct BlockRules {
    pub heading: bool,
    pub paragraph: bool,
}

impl Default for BlockRules {
    fn default() -> Self {
        Self {
            heading: true,
            paragraph: true,
        }
    }
}

pub fn replace(line: &str, rules: &mut ParserRules) -> Html {
    let mut line = line.trim().to_string();
    if rules.block.heading && let Some(h) = heading::from(&line) {
       line = h;
    } else if let Some(p) = paragraph::from(&line) {
        line = p;
    }
    line
}
