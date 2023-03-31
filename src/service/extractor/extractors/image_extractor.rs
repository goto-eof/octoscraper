use crate::service::{
    link_service::{add_base_url_if_not_present, add_http_if_not_present},
    validation_service::{contains_extension, is_same_domain_ext},
};
use select::{document::Document, predicate::Name};

use super::resource_extractor::ResourceExtractor;

pub struct ImageExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
}

impl ResourceExtractor for ImageExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn extract(&self, resource_str: &str) -> Vec<String> {
        let mut links: Vec<String> = Vec::new();

        if self.enabled {
            self.strategy_a(resource_str)
                .iter()
                .for_each(|elem| links.push(elem.to_string()));
        }

        return links;
    }
}

impl ImageExtractor {
    fn strategy_a(&self, resource_str: &str) -> Vec<String> {
        return Document::from(resource_str)
            .find(Name("img"))
            .filter_map(|n| n.attr("src"))
            .map(|item| item.to_string())
            .map(|link| add_http_if_not_present(link))
            .map(|link| add_base_url_if_not_present(&link, &self.domain))
            .filter_map(|link| {
                is_same_domain_ext(self.is_same_domain_enabled, self.domain.as_str(), &link)
            })
            .filter_map(|link| contains_extension(self.extensions.clone(), &link))
            .collect();
    }
}
