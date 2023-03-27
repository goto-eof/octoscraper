use crate::structure::{self, config_struct::Config, extension_filter_struct::ExtensionFilter};
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use structure::domain_filter_struct::DomainFilter;

use super::{
    download_service::download,
    validation_service::{contains_extension, is_same_domain},
};

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
        let item = &link_normalizer(item);
        if !processed.contains(item.as_str()) {
            processing.insert(item.to_string());
        }
    });

    let resources_links = extract_resources(&response_str, &domain_filter, &extension_filter).await;
    for resource_link in resources_links.iter() {
        let resource_link = &link_normalizer(resource_link);
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

pub fn link_normalizer(link: &str) -> String {
    let mut link = link.to_owned();
    if !link.starts_with("http:") && !link.starts_with("https:") {
        link = format!("http:{}", link);
    }
    return link;
}

pub fn initialize_download_directory(config: &Config) -> () {
    let resources_directory = format!("./{}", config.resources_directory);
    if !Path::new(&resources_directory).is_dir() {
        fs::create_dir(&resources_directory).unwrap();
    }
}

pub async fn extract_links(response_str: &str, domain_filter: &DomainFilter) -> Vec<String> {
    if response_str.contains("<html") {
        return Document::from(response_str)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|item| item.to_string())
            .filter_map(|link| is_same_domain(&domain_filter, domain_filter.is_same_domain, &link))
            .map(|link| normalize_link(&link))
            .collect();
    }
    return vec![];
}

pub fn normalize_link(link: &str) -> String{
    return link.replace(" ", "%20");
}

pub async fn extract_resources(
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
