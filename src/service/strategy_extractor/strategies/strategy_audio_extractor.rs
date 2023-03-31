use crate::service::strategy_extractor::strategy_resource_extractor::ResourceExtractor;

pub struct AudioExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
}

impl ResourceExtractor for AudioExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn extract(&self, resource_str: &str) -> Vec<String> {
        if self.enabled {
            let _test = resource_str;
            return vec![];
        }
        return vec![];
    }
}
