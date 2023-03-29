use super::{
    download_service::download,
    link_service::link_normalizer_add_http,
    resource_extractor_service::{extract_links, extract_resources},
};
use crate::{
    service::link_service::extract_fname,
    structure::{self, config_struct::Config, extension_filter_struct::ExtensionFilter},
};
use crossterm::{
    cursor::{self},
    QueueableCommand,
};
use reqwest::Url;
use std::{collections::HashSet, io::stdout};
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

    let mut resources_links: Vec<String> =
        extract_resources(&response_str, &domain_filter, &extension_filter)
            .await
            .iter()
            .map(|link| link_normalizer_add_http(link))
            .collect();

    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();
    loop {
        resources_links = download_all(
            resources_links,
            config,
            processed_resources,
            config.download_limit,
        )
        .await;

        if config.insistent_mode {
            if resources_links.is_empty() {
                break;
            } else {
                println!("Not all resources were downloaded correclty or there is a download limitation. Trying to download all remaining resources....\n\n");
            }
        } else {
            if resources_links.is_empty() {
                break;
            } else {
                println!(
                    "Not all resources were downloaded correctly. Retrying to download them...."
                )
            }
        }
    }
}

async fn download_all(
    resources_links: Vec<String>,
    config: &Config,
    processed_resources: &mut HashSet<String>,
    limit: i32,
) -> Vec<String> {
    let mut handlers = vec![];
    let mut i = 1;
    for resource_link in resources_links.iter() {
        println!("downloading: {}...", resource_link);
        handlers.push(download(resource_link, &config).await);
        i = i + 1;
        if i >= limit {
            break;
        }
    }
    for handler in handlers {
        if handler.is_some() {
            let handler_result = handler.unwrap().await;
            if handler_result.is_ok() {
                let handler_result = handler_result.unwrap();
                if handler_result.1 {
                    processed_resources.insert(extract_fname(&handler_result.0));
                    println!("{} downloaded successfully", handler_result.0);
                }
            }
        }
    }

    return resources_links
        .iter()
        .map(|link| link.to_owned())
        .filter(|link| !processed_resources.contains(extract_fname(link).as_str()))
        .collect();
}
