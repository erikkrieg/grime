mod block;
mod html;
mod inline;

pub fn transpile(markdown: &str) -> String {
    let output = markdown
        .lines()
        .filter(|l| !l.is_empty())
        .map(block::replace)
        .map(|l| inline::replace(&l))
        .intersperse("\n".into())
        .collect();
    output
}
