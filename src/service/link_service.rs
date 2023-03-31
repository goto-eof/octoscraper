use reqwest::Url;

pub fn link_normalizer_add_http(link: &str) -> String {
    let mut link = link.to_owned();
    if !link.starts_with("http:") && !link.starts_with("https:") {
        link = format!("http:{}", link);
    }
    return link;
}

pub fn normalize_link_replace_spaces(link: &str) -> String {
    return url_normalizer::normalize(Url::parse(link).unwrap())
        .unwrap()
        .as_str()
        .to_owned();
}

pub fn extract_fname_from_link(link: &str, alternative_file_name: Option<String>) -> String {
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

pub fn normalize_src(link: &str, domain: &str) -> String {
    let mut link = link.to_string();
    if !link.starts_with(&format!("http://{}", domain))
        && !link.starts_with(&format!("https://{}", domain))
        && !link.contains("http://")
        && !link.contains("https://")
    {
        link = format!("{}/{}", format!("http://{}", domain), link);
    } else if !link.starts_with("http:") && !link.starts_with("https:") {
        link = format!("{}{}", format!("http:{}", domain), link);
    }
    return link.to_owned();
}
