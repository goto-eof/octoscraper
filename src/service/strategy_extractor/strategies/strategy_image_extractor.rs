use crate::service::{
    strategy_extractor::strategy_resource_extractor::ResourceExtractor,
    validation_service::{contains_extension, is_same_domain_ext},
};
use select::{document::Document, predicate::Name};

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
        if self.enabled {
            return Document::from(resource_str)
                .find(Name("img"))
                .filter_map(|n| n.attr("src"))
                .map(|item| item.to_string())
                .filter_map(|link| {
                    is_same_domain_ext(self.is_same_domain_enabled, self.domain.as_str(), &link)
                })
                .filter_map(|link| contains_extension(self.extensions.clone(), &link))
                .collect();
        }
        return vec![];
    }
}
