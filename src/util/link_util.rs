use reqwest::Url;

pub fn normalize_link_replace_spaces(link: &str) -> Option<String> {
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
pub fn add_base_url_if_not_present(link: &str, domain: &str) -> String {
    let base_url = get_domain_base_url_string(domain);
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

pub fn get_domain_base_url_string(domain: &str) -> String {
    let url = Url::parse(domain).unwrap();
    let base_url = base_url(url).unwrap().as_str().to_owned();
    base_url
}

pub fn add_http_if_not_present(link: String) -> String {
    if !link.starts_with("//") && !link.starts_with("http://") && !link.starts_with("https://") {
        return format!("http://{}", link);
    }
    if link.starts_with("//") && !link.starts_with("http:") && !link.starts_with("https:") {
        return format!("http:{}", link);
    }
    return link;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_http_slash_if_not_present_test() {
        let result = add_http_if_not_present("ciao.com".to_owned());
        assert_eq!(result, "http://ciao.com");
    }

    #[test]
    fn add_http_if_not_present_test() {
        let result = add_http_if_not_present("//ciao.com".to_owned());
        assert_eq!(result, "http://ciao.com");
    }

    #[test]
    fn add_base_url_if_not_present_test() {
        let link = "http://www.dodu.it";
        let domain = "http://www.dodu.it";
        let result = add_base_url_if_not_present(link, domain);
        assert_eq!("http://www.dodu.it", result);
    }

    #[test]
    fn add_base_url_if_not_present_test2() {
        let link = "resource/something.png";
        let domain = "http://www.dodu.it";
        let result = add_base_url_if_not_present(link, domain);
        assert_eq!("http://www.dodu.it/resource/something.png", result);
    }

    #[test]
    fn normalize_link_replace_spaces_test() {
        let link = "http://dodu.it/this is a test";
        let result = normalize_link_replace_spaces(link).unwrap();
        assert_eq!("http://dodu.it/this%20is%20a%20test", result);
    }

    #[test]
    #[should_panic]
    fn normalize_link_replace_spaces_test2() {
        let link = "dodu.it/this is a test";
        let result = normalize_link_replace_spaces(link).unwrap();
        assert_eq!("dodu.it/this%20is%20a%20test", result);
    }

    #[test]
    fn extract_fname_from_link_test() {
        let link = "http://www.dodu.it/file.png";
        let result = extract_fname_from_link(link, None);
        assert_eq!("file.png", result);
    }

    #[test]
    fn extract_fname_from_link_test2() {
        let link = "http://www.dodu.it/file";
        let result = extract_fname_from_link(link, None);
        assert_eq!("file", result);
    }
}
