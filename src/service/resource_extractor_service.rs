use super::{link_service::normalize_link_replace_spaces, validation_service::is_same_domain};
use crate::structure::domain_filter_struct::DomainFilter;
use select::{document::Document, predicate::Name};

pub async fn extract_links(response_str: &str, domain_filter: &DomainFilter) -> Vec<String> {
    if response_str.contains("<html") {
        return Document::from(response_str)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|item| item.to_string())
            .filter_map(|link| is_same_domain(&domain_filter, domain_filter.is_same_domain, &link))
            .map(|link| normalize_link_replace_spaces(&link))
            .collect();
    }
    return vec![];
}
