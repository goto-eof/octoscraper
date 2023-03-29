use reqwest::Url;

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

pub fn extract_fname(link: &str, alternative_file_name: Option<String>) -> String {
    let alternative_name = if alternative_file_name.is_some() {
        alternative_file_name.unwrap()
    } else {
        "no_filename".to_string()
    };
    return Url::parse(link)
        .unwrap()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or(&alternative_name)
        .to_string();
}
