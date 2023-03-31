use crate::service::link_service::{normalize_link_replace_spaces, normalize_src};
use crate::service::validation_service::is_same_domain_ext;
use select::{document::Document, predicate::Name};

use super::resource_extractor::ResourceExtractor;

pub struct LinkExtractor {
    pub enabled: bool,
    pub is_same_domain_enabled: bool,
    pub domain: String,
}

impl ResourceExtractor for LinkExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn extract(&self, resource_str: &str) -> Vec<String> {
        if self.enabled {
            if resource_str.contains("<html") {
                return Document::from(resource_str)
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .map(|item| item.to_string())
                    .map(|link| normalize_src(&link, &self.domain))
                    .filter_map(|link| {
                        is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link)
                    })
                    .map(|link| normalize_link_replace_spaces(&link))
                    .collect();
            }
            return vec![];
        }
        return vec![];
    }
}
