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
    let mut html_out = html::Html::new();
    let mut lines = markdown.lines().filter(|l| !l.trim().is_empty()).peekable();
    while let Some(line) = lines.next() {
        let mut rules = ParserRules::default();
        let line = line.trim();
        let is_last_line = lines.peek().is_none();
        let (line, prefix, next_line) = section::replace(line, &mut rules, lines.peek());
        // TODO: move this into section::replace once done testing. Will require
        // moderate refactoring.
        let line = section::code::replace(&line, &mut rules, &mut lines);
        if !line.is_empty() {
            let line = block::replace(&line, &mut rules);
            let line = inline::replace(&line, &mut rules);
            html_out += &format!("{}{line}", prefix.unwrap_or("".into()));
            if !is_last_line || next_line.is_some() {
                html_out += "\n";
            }
        }
        if let Some(l) = next_line {
            html_out += &l;
            if !is_last_line {
                html_out += "\n";
            }
        }
    }
    html_out
}
