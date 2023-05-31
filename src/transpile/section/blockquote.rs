use crate::transpile::{html::Html, ParserRules};

fn is_blockquote(s: &str) -> bool {
    matches!(s.chars().next(), Some('>'))
}

fn trim_blockquote(s: &str) -> &str {
    // s[s.chars().position(|c| c == '>').unwrap_or(0)..].trim()
    s[1..].trim()
}

fn toggle_blockquote(enter: bool, rules: &mut ParserRules) {
    rules.section.code = !enter;
    rules.block.heading = !enter;
    rules.block.paragraph = !enter;
}

pub fn replace(
    line: &str,
    rules: &mut ParserRules,
    next_line: Option<&&str>,
) -> (Html, Option<Html>, Option<Html>) {
    let line = line;
    let mut html_out = (line.to_string(), None, None);
    let next_is_blockquote = is_blockquote(next_line.unwrap_or(&"").trim());
    if is_blockquote(line) {
        html_out.0 = trim_blockquote(line).to_string();
        html_out.1 = Some("\t".to_string());
        toggle_blockquote(true, rules);
        if !next_is_blockquote {
            html_out.2 = Some("</blockquote>".into());
        }
    } else {
        toggle_blockquote(false, rules);
        if next_is_blockquote {
            html_out.2 = Some("<blockquote>".into());
        }
    }
    html_out
}
