pub fn is_same_domain_ext(enabled: bool, domain: &str, link: &str) -> Option<String> {
    if enabled {
        if link.contains(domain) {
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
