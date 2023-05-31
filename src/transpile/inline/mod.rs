use super::{html::Html, ParserRules};
use regex::Regex;

pub mod bold;
pub mod italic;

#[derive(Debug)]
pub struct InlineRules {
    pub bold: bool,
    pub italic: bool,
}

impl Default for InlineRules {
    fn default() -> Self {
        Self {
            bold: true,
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

pub fn replace(text: &str, rules: &mut ParserRules) -> String {
    let mut text = text.trim().to_string();
    if rules.inline.bold {
        text = bold::replace(&text);
    }
    if rules.inline.italic {
        text = italic::replace(&text);
    }
    if rules.inline.bold && rules.inline.italic {
        text = hacky_fix(&text);
    }
    text
}
