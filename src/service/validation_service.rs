use crate::structure;
use structure::domain_filter_struct::DomainFilter;

pub fn is_same_domain(
    domain_filter: &DomainFilter,
    is_same_domain: bool,
    link: &str,
) -> Option<String> {
    if is_same_domain {
        if link.contains(&domain_filter.domain) {
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
