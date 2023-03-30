use select::{document::Document, predicate::Name};

use crate::{
    service::validation_service::{contains_extension, is_same_domain},
    structure::{
        config_struct::Config, domain_filter_struct::DomainFilter,
        extension_filter_struct::ExtensionFilter,
    },
};

use super::strategy_resource_extractor::ResourceExtractor;

pub struct ImageExtractor {}

impl ResourceExtractor for ImageExtractor {
    fn extract(
        &self,
        config: &Config,
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
}
