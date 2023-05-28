use std::char;

type Html = String;

pub fn transpile(markdown: &str) -> String {
    let output = markdown
        .lines()
        .fold(String::new(), |acc, l| acc + &convert_line(l));
    output
}

fn convert_line(line: &str) -> String {
    let l = line.trim();
    let index = l.find(char::is_whitespace).unwrap_or(0);
    let first = &l[0..index];
    match first {
        "#" => tag("h1", Some(&l[(index + 1)..]), None),
        "##" => tag("h2", Some(&l[(index + 1)..]), None),
        "###" => tag("h3", Some(&l[(index + 1)..]), None),
        "####" => tag("h4", Some(&l[(index + 1)..]), None),
        "#####" => tag("h5", Some(&l[(index + 1)..]), None),
        "######" => tag("h6", Some(&l[(index + 1)..]), None),
        _ => l.to_string(),
    }
}

fn tag(tag_type: &str, content: Option<&str>, attributes: Option<&str>) -> Html {
    let open_tag = match attributes {
        Some(attr) => format!("<{tag_type} {attr}>"),
        None => format!("<{tag_type}>"),
    };
    match content {
        Some(c) => format!("{open_tag}{}</{tag_type}>", c.trim()),
        None => open_tag,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_1() {
        assert_eq!(transpile("# Heading 1"), "<h1>Heading 1</h1>".to_string())
    }

    #[test]
    fn test_heading_2() {
        assert_eq!(transpile("## Heading 2"), "<h2>Heading 2</h2>".to_string())
    }

    #[test]
    fn test_heading_3() {
        assert_eq!(transpile("### Heading 3"), "<h3>Heading 3</h3>".to_string())
    }

    #[test]
    fn test_heading_4() {
        assert_eq!(
            transpile("#### Heading 4"),
            "<h4>Heading 4</h4>".to_string()
        )
    }

    #[test]
    fn test_heading_5() {
        assert_eq!(
            transpile("##### Heading 5"),
            "<h5>Heading 5</h5>".to_string()
        )
    }

    #[test]
    fn test_heading_6() {
        assert_eq!(
            transpile("###### Heading 6"),
            "<h6>Heading 6</h6>".to_string()
        )
    }
}
