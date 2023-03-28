use super::{
    link_normalizer_service::normalize_link_replace_spaces,
    validation_service::{contains_extension, is_same_domain},
};
use crate::structure::{
    domain_filter_struct::DomainFilter, extension_filter_struct::ExtensionFilter,
};
use select::{document::Document, predicate::Name};

pub async fn extract_resources(
    resource_str: &str,
    domain_filter: &DomainFilter,
    extension_filter: &ExtensionFilter,
) -> Vec<String> {
    return Document::from(resource_str)
        .find(Name("img"))
        .filter_map(|n| n.attr("src"))
        .map(|item| item.to_string())
        .filter_map(|link| {
            is_same_domain(
                &domain_filter,
                extension_filter.is_resource_same_domain,
                &link,
            )
        })
        .filter_map(|link| contains_extension(extension_filter.extensions.clone(), &link))
        .collect();
}

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
