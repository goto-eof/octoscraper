use reqwest::Url;

use super::link_util::base_url;

pub fn is_same_domain_ext(enabled: bool, domain: &str, link: &str) -> Option<String> {
    if enabled {
        let domain_base = Url::parse(domain).unwrap();
        let link_base = Url::parse(link).unwrap();
        let base_domain = base_url(domain_base).unwrap();
        let base_link = base_url(link_base).unwrap();
        if base_domain.eq(&base_link) {
            return Some(link.to_string());
        } else {
            return None;
        };
    }
    return Some(link.to_string());
}

pub fn contains_extension(extensions: Vec<String>, link: &str) -> Option<String> {
    for extension in extensions {
        if link.ends_with(&extension) {
            return Some(link.to_string());
        }
    }
    return None;
}
