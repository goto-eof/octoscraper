use super::{
    download_service::download,
    link_normalizer_service::link_normalizer_add_http,
    resource_extractor_service::{extract_links, extract_resources},
};
use crate::structure::{self, config_struct::Config, extension_filter_struct::ExtensionFilter};
use std::collections::HashSet;
use structure::domain_filter_struct::DomainFilter;

pub async fn extract_links_and_process_data(
    link: &str,
    config: &Config,
    processing: &mut HashSet<String>,
    processed: &mut HashSet<String>,
    processed_resources: &mut HashSet<String>,
    domain_filter: &DomainFilter,
    extension_filter: &mut ExtensionFilter,
) {
    let response_str = reqwest::get(link).await.unwrap().text().await.unwrap();

    let extracted_links = extract_links(&response_str, &domain_filter).await;
    extracted_links.iter().for_each(|item| {
        let item = &link_normalizer_add_http(item);
        if !processed.contains(item.as_str()) {
            processing.insert(item.to_string());
        }
    });

    let resources_links: Vec<String> =
        extract_resources(&response_str, &domain_filter, &extension_filter)
            .await
            .iter()
            .map(|link| link_normalizer_add_http(link))
            .collect();

    let mut handlers = vec![];
    for resource_link in resources_links.iter() {
        handlers.push(download(resource_link, &config).await);
    }

    for handler in handlers {
        let handler_result = handler.unwrap().await;
        if handler_result.is_ok() {
            let handler_result = handler_result.unwrap();
            if handler_result.1 {
                processed_resources.insert(handler_result.0.to_owned());
            }
        }
    }
}
