pub mod heading;
pub mod paragraph;

use crate::transpile::html::Html;
pub fn replace(line: &str) -> Html {
    let l = line.trim();
    heading::from(l).unwrap_or(paragraph::from(l).unwrap_or(l.to_string()))
}
