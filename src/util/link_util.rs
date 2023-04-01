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

pub fn add_base_url_if_not_present(link: &str, domain: &str, processing_page_link: &str) -> String {
    let base_url = get_domain_base_url_string(domain);
    let mut link = link.to_string();

    if !link.starts_with(&base_url)
        && !link.starts_with(&base_url)
        && !link.contains("http://")
        && !link.contains("https://")
    {
        if link.starts_with("//") {
            // substring
            link = link[2..link.len()].to_string();
            link = format!("http://{}", link);
            return link.to_owned();
        }

        if link.starts_with("/") {
            // substring
            link = link[1..link.len()].to_string();
        }
        link = format!("{}{}", base_url_path(processing_page_link), link);
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

pub fn has_extension(link: &str, extensions: Vec<String>) -> Option<String> {
    for extension in extensions.iter() {
        if link.ends_with(extension) {
            return Some(link.to_string());
        }
    }
    return None;
}

pub fn base_url_path(link: &str) -> String {
    let u = Url::parse(link).unwrap();
    return u.join("./").unwrap().as_str().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_base_url_if_not_present_test() {
        let link = "http://www.dodu.it/file.png";
        let domain = "http://www.dodu.it";
        let processing_page_link = "http://www.dodu.it/test/ciao-mondo.jsp";
        let result = add_base_url_if_not_present(link, domain, processing_page_link);
        assert_eq!("http://www.dodu.it/file.png", result);
    }

    #[test]
    fn add_base_url_if_not_present_test2() {
        let link = "resource/something.png";
        let domain = "http://www.dodu.it";
        let processing_page_link = "http://www.dodu.it/test/ciao-mondo.jsp";
        let result = add_base_url_if_not_present(link, domain, processing_page_link);
        assert_eq!("http://www.dodu.it/test/resource/something.png", result);
    }

    #[test]
    fn add_base_url_if_not_present_test3() {
        let link = "/resource/something.png";
        let domain = "http://www.dodu.it";
        let processing_page_link = "http://www.dodu.it/test/ciao-mondo.jsp";
        let result = add_base_url_if_not_present(link, domain, processing_page_link);
        assert_eq!("http://www.dodu.it/test/resource/something.png", result);
    }

    #[test]
    fn add_base_url_if_not_present_test4() {
        let link = "//dodu.it/resource/something.png";
        let domain = "http://www.dodu.it";
        let processing_page_link = "http://www.dodu.it/test/ciao-mondo.jsp";
        let result = add_base_url_if_not_present(link, domain, processing_page_link);
        assert_eq!("http://dodu.it/resource/something.png", result);
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
