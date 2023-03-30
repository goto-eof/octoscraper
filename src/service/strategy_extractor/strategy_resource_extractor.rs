use crate::structure::{
    config_struct::Config, domain_filter_struct::DomainFilter,
    extension_filter_struct::ExtensionFilter,
};

pub trait ResourceExtractor {
    fn extract(
        &self,
        config: &Config,
        resource_str: &str,
        domain_filter: &DomainFilter,
        extension_filter: &ExtensionFilter,
    ) -> Vec<String>;
}
