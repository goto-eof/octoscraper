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

    let resources_links = extract_resources(&response_str, &domain_filter, &extension_filter).await;
    for resource_link in resources_links.iter() {
        let resource_link = &link_normalizer_add_http(resource_link);
        if !processed_resources.contains(resource_link) {
            loop {
                let result = download(resource_link, &config).await;
                if result {
                    processed_resources.insert(resource_link.to_owned());
                    break;
                } else {
                    println!("retrying to download resource ({})...", resource_link);
                }
            }
        }
    }
}
