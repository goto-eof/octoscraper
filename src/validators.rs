pub fn is_same_domain(domain: &str, link: &str) -> Option<String> {
    return if link.contains(domain) {
        Some(link.to_string())
    } else {
        None
    };
}
pub fn contains_extension(extensions: Vec<String>, link: &str) -> Option<String> {
    for extension in extensions {
        if link.ends_with(&extension) {
            return Some(link.to_string());
        }
    }
    return None;
}
