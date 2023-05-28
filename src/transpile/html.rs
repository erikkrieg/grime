pub type Html = String;

pub fn create(tag_name: &str, inner: Option<&str>, attributes: Option<&str>) -> Html {
    let open_tag = match attributes {
        Some(attr) => format!("<{tag_name} {attr}>"),
        None => format!("<{tag_name}>"),
    };
    match inner {
        Some(c) => format!("{open_tag}{}</{tag_name}>", c.trim()),
        None => open_tag,
    }
}
