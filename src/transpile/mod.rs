mod block;
mod html;
mod inline;
mod section;

#[derive(Debug, Default)]
pub struct ParserRules {
    pub section: section::SectionRules,
    pub block: block::BlockRules,
    pub inline: inline::InlineRules,
}

pub fn parse(markdown: &str) -> html::Html {
    let mut rules = ParserRules::default();
    let mut html_out = html::Html::new();
    let mut lines = markdown
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .peekable();
    while let Some(line) = lines.next() {
        let (line, prefix, next_line) = section::replace(line, &mut rules, lines.peek());
        if !line.is_empty() {
            let line = block::replace(&line, &mut rules);
            let line = inline::replace(&line, &mut rules);
            html_out += &format!("{}{line}", prefix.unwrap_or("".into()));
            // Avoid newline EOF
            if lines.peek().is_some() || next_line.is_some() {
                html_out += "\n";
            }
        }
        if let Some(l) = next_line {
            html_out += &format!("{l}\n");
        }
    }
    html_out
}
