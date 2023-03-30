use super::{
    download_service::download,
    link_service::link_normalizer_add_http,
    resource_extractor_service::{extract_links, extract_resources},
};
use crate::{
    service::{
        file_service::{file_delete, file_rename},
        link_service::extract_fname_from_link,
    },
    structure::{
        self, config_struct::Config, extension_filter_struct::ExtensionFilter,
        processed_hash_struct::ProcessedHash, processed_struct::Processed,
    },
};
use crossterm::{
    cursor::{self},
    QueueableCommand,
};
use std::{collections::HashSet, io::stdout};
use structure::domain_filter_struct::DomainFilter;
/**
 * è necessario consentire all'utente la possibilità di preservare i nomi originali. Quindi:
 * la lista dei file scaricati è costituita dagli md5(file)
 * 1-scarico file con nome random
 * 2-alla fine del download calcolo il suo md5(file)
 * 3-salvo nella lista dei processati
 * 4-rinnomino il file con il suo nome originale, se vi è un altro file con lo stesso nome aggiungo un numero finale
 * (stessa radice del nome, contenuti diversi)
 */
pub async fn extract_links_and_process_data(
    link: &str,
    config: &Config,
    processing: &mut HashSet<String>,
    processed: &mut HashSet<String>,
    processed_resources: &mut Processed,
    processed_resources_hash: &mut ProcessedHash,
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
            processed_resources_hash,
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
    processed_resources: &mut Processed,
    processed_resources_hash: &mut ProcessedHash,
    limit: i32,
) -> Vec<String> {
    let mut handlers = vec![];
    let mut i = 1;
    for resource_link in resources_links.iter() {
        if !processed_resources.was_already_processed(&resource_link) {
            println!("downloading: {}...", resource_link);
            handlers.push(download(resource_link, &config).await);
            i = i + 1;
            if i >= limit {
                break;
            }
        }
    }
    for handler in handlers {
        if handler.is_some() {
            let handler_result = handler.unwrap().await;
            if handler_result.is_ok() {
                let handler_result = handler_result.unwrap();
                if handler_result.1 {
                    processed_resources.push(&handler_result.0);
                    if processed_resources_hash.was_already_processed(&handler_result.2) {
                        file_delete(&handler_result.2);
                        println!("Huston! We have already another file with the same hash. This file will be discarded. Details: {}", handler_result.0)
                    } else {
                        processed_resources_hash.push(&handler_result.2);
                        file_rename(
                            config,
                            &handler_result.2,
                            &extract_fname_from_link(&handler_result.0, None),
                        );
                        println!("downloaded SUCCESSFULLY: {}", handler_result.0);
                    }
                } else {
                    file_delete(&handler_result.2);
                    println!("download failed: {}", handler_result.0);
                }
            }
        }
    }

    return resources_links
        .iter()
        .map(|link| link.to_owned())
        .filter(|link| !processed_resources.was_already_processed(&link))
        .collect();
}
