use reqwest::Url;

pub fn link_normalizer_add_http(link: &str) -> String {
    let mut link = link.to_owned();
    if !link.starts_with("http:") && !link.starts_with("https:") {
        link = format!("http:{}", link);
    }
    return link;
}

pub fn normalize_link_replace_spaces(link: &str) -> Option<String> {
    // println!("normalizing: {}", link);
    let parsed_url = Url::parse(link);
    if parsed_url.is_ok() {
        return Some(
            url_normalizer::normalize(parsed_url.unwrap())
                .unwrap()
                .as_str()
                .to_owned(),
        );
    }
    return None;
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

// cases
// <a href="download/midi_files/Armageddon1.mid">
// <a href="http://ininternet.org/download/midi_files/aladdin.mid">
pub fn normalize_src(link: &str, domain: &str) -> String {
    println!("{}", domain);
    let url = Url::parse(domain).unwrap();
    let base_url = base_url(url).unwrap().as_str().to_owned();
    println!("base: {}", base_url);
    let mut link = link.to_string();
    if !link.starts_with(&base_url)
        && !link.starts_with(&base_url)
        && !link.contains("http://")
        && !link.contains("https://")
    {
        link = format!("{}{}", base_url, link);
    }
    return link.to_owned();
}

pub fn base_url(mut url: Url) -> Option<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return None;
        }
    }
    url.set_query(None);
    Some(url)
}
