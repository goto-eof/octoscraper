pub fn link_normalizer_add_http(link: &str) -> String {
    let mut link = link.to_owned();
    if !link.starts_with("http:") && !link.starts_with("https:") {
        link = format!("http:{}", link);
    }
    return link;
}

pub fn normalize_link_replace_spaces(link: &str) -> String {
    return link.replace(" ", "%20");
}
