use select::{document::Document, predicate::Name};

use crate::util::{
    link_util::{add_base_url_if_not_present, has_extension, normalize_link_replace_spaces},
    validation_util::is_same_domain_ext,
};

pub trait ResourceExtractor {
    fn enabled(&mut self, enabled: bool);
    fn extract(&self, resource_str: &str) -> Vec<String>;
}

pub fn strategy_a_common_extractor(
    resource_str: &str,
    extensions: Vec<String>,
    domain: &str,
    is_same_domain_enabled: bool,
    processing_page_link: String,
) -> Vec<String> {
    return Document::from(resource_str)
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|item| item.to_string())
        .filter_map(|link| has_extension(&link, extensions.clone()))
        .map(|link| add_base_url_if_not_present(&link, domain, &processing_page_link))
        .filter_map(|link| normalize_link_replace_spaces(&link))
        .filter_map(|link| is_same_domain_ext(is_same_domain_enabled, domain, &link))
        .collect();
}
