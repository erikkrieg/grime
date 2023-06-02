use crate::transpile::{html::Html, ParserRules};
use std::iter::Peekable;

pub fn replace(
    line: &str,
    rules: &mut ParserRules,
    lines: &mut Peekable<impl Iterator<Item = &str>>,
) -> Html {
    let mut html_out = line.to_string();
    if !rules.section.code {
        return html_out;
    }

    if line == "```" {
        rules.block.paragraph = false;
        html_out = "<code>\n".into();
        html_out += &lines
            .take_while(|l| l.trim() != "```")
            .fold(Html::new(), |h, l| format!("{h}{}\n", l.trim_end()));
        html_out += "</code>";
    }
    html_out
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn setup(markdown: &str) -> (&str, ParserRules, Peekable<impl Iterator<Item = &str>>) {
        let rules = ParserRules::default();
        let mut lines = markdown.lines().peekable();
        let line = lines.next().unwrap();
        (line, rules, lines)
    }

    #[test]
    fn code_section_replace() {
        let markdown = "```
    #[test]
    fn code_section_replace() { ... }
```";
        let html = "<code>
    #[test]
    fn code_section_replace() { ... }
</code>";
        let (line, mut rules, mut lines) = setup(markdown);
        let parsed = replace(line, &mut rules, &mut lines);
        assert_eq!(parsed, html);
    }

    #[test]
    fn ingores_lines_outside_code_section() {
        let markdown = "```
    #[test]
    fn code_section_replace() { ... }
```
below";
        let html = "<code>
    #[test]
    fn code_section_replace() { ... }
</code>";
        let (line, mut rules, mut lines) = setup(markdown);
        let parsed = replace(line, &mut rules, &mut lines);
        assert_eq!(parsed, html);
    }

    #[test]
    // This is an implicit feature the results from the current implementation.
    // Changing this behavior might be a breaking change.
    fn code_section_no_closing_backticks() {
        let markdown = "```
    #[test]
    fn code_section_replace() { ... }";
        let html = "<code>
    #[test]
    fn code_section_replace() { ... }
</code>";
        let (line, mut rules, mut lines) = setup(markdown);
        let parsed = replace(line, &mut rules, &mut lines);
        assert_eq!(parsed, html);
    }
}
