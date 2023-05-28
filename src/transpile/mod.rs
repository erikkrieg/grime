mod heading;
mod html;
mod paragraph;

pub fn transpile(markdown: &str) -> String {
    let output = markdown
        .lines()
        .filter(|l| !l.is_empty())
        .map(convert_line)
        .intersperse("\n".into())
        .collect();
    output
}

fn convert_line(line: &str) -> String {
    let l = line.trim();
    heading::from(l).unwrap_or(paragraph::from(l).unwrap_or(l.to_string()))
}
