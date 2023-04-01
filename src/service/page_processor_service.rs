use super::{
    download_service::download,
    extractor::{
        extractor_service::{retrieve_extractors, ExtractorType},
        extractors::{
            audio_link_extractor::AudioLinkExtractor, image_link_extractor::ImageLinkExtractor,
            link_extractor::LinkExtractor, other_link_extractor::OtherFileLinkExtractor,
            resource_extractor::ResourceExtractor, video_link_extractor::VideoLinkExtractor,
        },
    },
};
use crate::{
    structure::{
        application_settings::ApplicationSettings, config_struct::Config,
        processed_hash_struct::ProcessedHash, processed_struct::Processed,
    },
    util::{
        file_util::{file_delete, file_len_less_than, file_rename},
        link_util::{add_base_url_if_not_present, extract_fname_from_link},
    },
};
use std::collections::{HashMap, HashSet};

pub async fn extract_links_and_process_data(
    application_settings: &ApplicationSettings,
    link: &str,
    config: &Config,
    processing: &mut HashSet<String>,
    processed: &mut HashSet<String>,
    processed_resources: &mut Processed,
    processed_resources_hash: &mut ProcessedHash,
) -> (bool, String) {
    let response_str = reqwest::get(link).await;
    if response_str.is_ok() {
        let response_str = response_str.unwrap().text().await.unwrap();

        let mut resource_links_type: HashMap<String, String> = HashMap::new();

        if !config.process_only_root {
            let link_extractor = LinkExtractor {
                domain: config.website.clone(),
                enabled: true,
                is_same_domain_enabled: config.processing_same_domain,
                processing_page_link: link.to_owned(),
            };
            let extracted_links = link_extractor.extract(&response_str);
            extracted_links.iter().for_each(|item| {
                let item = &add_base_url_if_not_present(&item.to_string(), &config.website, link);
                if !processed.contains(item.as_str()) {
                    processing.insert(item.to_string());
                }
            });
        }

        let extractors: Vec<ExtractorType> = retrieve_extractors(config, link);
        let mut resources_links: HashSet<String> = HashSet::new();

        extractors.iter().for_each(|extractor| {
            extractor
                .extract(&response_str)
                .iter()
                .map(|link| add_base_url_if_not_present(&link.to_string(), &config.website, link))
                .for_each(|link| {
                    resources_links.insert(link.to_owned());
                    resource_links_type.insert(link, extractor.get_name());
                })
        });

        loop {
            resources_links = download_all(
                application_settings,
                resources_links,
                &resource_links_type,
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
        return (true, format!("{} processed successfully", link));
    } else {
        return (
            false,
            format!("Error processing link: {}\n{:?}", link, response_str.err()),
        );
    }
}

async fn download_all(
    application_settings: &ApplicationSettings,
    resources_links: HashSet<String>,
    resource_links_type: &HashMap<String, String>,
    config: &Config,
    processed_resources: &mut Processed,
    processed_resources_hash: &mut ProcessedHash,
    limit: i32,
) -> HashSet<String> {
    let mut handlers = vec![];
    let mut i = 1;
    for resource_link in resources_links.iter() {
        if !processed_resources.was_already_processed(&resource_link) {
            println!("downloading: {}...", resource_link);
            handlers.push(download(application_settings, resource_link, &config).await);
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
                let handler_link = handler_result.0;
                let is_success = handler_result.1;
                let handler_file = handler_result.2;
                if is_success {
                    let acceptable_size = retrieve_size_for_file_type(
                        resource_links_type.get(&handler_link).unwrap(),
                        config,
                    );
                    processed_resources.push(&handler_link);
                    if processed_resources_hash.was_already_processed(&handler_file) {
                        file_delete(&handler_file);
                        println!("Huston! We have already another file with the same hash. This file will be discarded. Details: {}", handler_link)
                    } else if file_len_less_than(&handler_file, acceptable_size) {
                        file_delete(&handler_file);
                        println!(
                            "File discarded because it's length is less than expected ({}). Details: {}",
                            acceptable_size,
                            handler_link
                        )
                    } else {
                        processed_resources_hash.push(&handler_file);
                        file_rename(
                            config,
                            &handler_file,
                            &extract_fname_from_link(&handler_link, None),
                        );
                        println!("downloaded SUCCESSFULLY: {}", handler_link);
                    }
                } else {
                    file_delete(&handler_file);
                    println!("download failed: {}", handler_link);
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

fn retrieve_size_for_file_type(extractor_type: &str, config: &Config) -> u64 {
    match extractor_type {
        ImageLinkExtractor::EXTRACTOR_NAME => config.image_extractor_minimum_size,
        AudioLinkExtractor::EXTRACTOR_NAME => config.audio_extractor_minimum_size,
        VideoLinkExtractor::EXTRACTOR_NAME => config.video_extractor_minimum_size,
        OtherFileLinkExtractor::EXTRACTOR_NAME => config.other_file_extractor_minimum_size,
        _ => u64::MAX,
    }
}
