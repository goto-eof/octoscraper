use select::{document::Document, predicate::Name};

use crate::{
    service::{
        strategy_extractor::strategy_resource_extractor::ResourceExtractor,
        validation_service::{contains_extension, is_same_domain},
    },
    structure::{
        config_struct::Config, domain_filter_struct::DomainFilter,
        extension_filter_struct::ExtensionFilter,
    },
};

pub struct VideoExtractor {}

// TODO
impl ResourceExtractor for VideoExtractor {
    fn extract(
        &self,
        config: &Config,
        resource_str: &str,
        domain_filter: &DomainFilter,
        extension_filter: &ExtensionFilter,
    ) -> Vec<String> {
        return Document::from(resource_str)
            .find(Name("source"))
            .filter_map(|n| n.attr("src"))
            .map(|item| item.to_string())
            .filter_map(|link| {
                is_same_domain(
                    &domain_filter,
                    extension_filter.is_resource_same_domain,
                    &link,
                )
            })
            .filter_map(|link| {
                contains_extension(vec![".mp4".to_string(), ".ogg".to_string()], &link)
            })
            .collect();
    }
}
