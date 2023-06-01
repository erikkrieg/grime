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
        // TODO: Check and error if code block doesn't close.
        html_out += &lines
            .take_while(|l| l.trim() != "```")
            .fold(Html::new(), |h, l| {
                println!("line: {l}");
                format!("{h}{}\n", l.trim_end())
            });
        // Currently this means that hitting the EOF will also close the code.
        // If this happens unintentionally it should be a pretty clear so going
        // to let this slide for now.
        html_out += "</code>";
    }
    html_out
}
