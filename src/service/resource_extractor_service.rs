use super::{link_service::normalize_link_replace_spaces, validation_service::is_same_domain_ext};
use crate::structure::config_struct::Config;
use select::{document::Document, predicate::Name};

pub async fn extract_links(config: &Config, response_str: &str) -> Vec<String> {
    if response_str.contains("<html") {
        return Document::from(response_str)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|item| item.to_string())
            .filter_map(|link| {
                is_same_domain_ext(config.processing_same_domain, &config.website, &link)
            })
            .map(|link| normalize_link_replace_spaces(&link))
            .collect();
    }
    return vec![];
}
