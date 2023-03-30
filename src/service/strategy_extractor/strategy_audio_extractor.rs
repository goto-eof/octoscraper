use super::strategy_resource_extractor::ResourceExtractor;
use crate::structure::{
    config_struct::Config, domain_filter_struct::DomainFilter,
    extension_filter_struct::ExtensionFilter,
};

pub struct AudioExtractor {}

impl ResourceExtractor for AudioExtractor {
    fn extract(
        &self,
        config: &Config,
        resource_str: &str,
        domain_filter: &DomainFilter,
        extension_filter: &ExtensionFilter,
    ) -> Vec<String> {
        return vec![];
    }
}
