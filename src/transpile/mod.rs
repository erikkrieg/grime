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

/*
 * This is needs a major refactor as it is brittle spaghetti.
 * Blockquote is all mixed into everything in a way that could be duplicated
 * by some other multi-line blocks. The other multi-line blocks:
 * - blockquote
 *  - can contain other block elements (only block that can do this)
 *  - can contain inline elements
 * - list (ordered and unordered)
 *  - can contain inline elements
 * - code
 *  - can contain line breaks
 *  - special: maybe syntax highlighting for other langs
 * - horizontal rules
 *  - not sure if I want this to be <section> or just <hr>
 */
pub fn parse(md: &str) -> html::Html {
    let mut lines = md.lines().peekable();
    let mut body = String::new();
    let mut blockquote = false;
    while let Some(line) = lines.next() {
        let line = line.trim();
        let first_char = line.chars().next();
        match first_char {
            Some('>') => {
                if !blockquote {
                    body += "<blockquote>\n";
                    blockquote = true;
                }
            }
            Some('-') => todo!("Handle unordered list"),
            _ => (),
        };
        let line = inline::replace(if blockquote {
            line[1..].trim_start()
        } else {
            line
        });
        let mut line = block::replace(&line);
        if blockquote {
            line = format!("\t{line}");
        }
        if !line.trim().is_empty() {
            body += &line;
            body += "\n";
        }
        if blockquote {
            blockquote = matches!(lines.peek().unwrap_or(&"").chars().next(), Some('>'));
            if !blockquote {
                body += "</blockquote>\n";
            }
        }
    }
    body
}
