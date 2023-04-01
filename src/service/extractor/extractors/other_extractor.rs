use super::resource_extractor::{strategy_a_common_extractor, ResourceExtractor};

pub struct OtherFileExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
    pub processing_page_link: String,
}

impl ResourceExtractor for OtherFileExtractor {
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

impl OtherFileExtractor {
    fn strategy_a(&self, resource_str: &str) -> Vec<String> {
        return strategy_a_common_extractor(
            resource_str,
            self.extensions.clone(),
            &self.domain,
            self.is_same_domain_enabled,
            self.processing_page_link.to_owned(),
        );
    }
}
