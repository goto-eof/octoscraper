use crate::page_processor::initialize_download_directory;
use crate::structs::{DomainFilter, ExtensionFilter};
use crate::{configuration::Config, page_processor::extract_links_and_process_data};
use config_file::FromConfigFile;
use std::collections::HashSet;

mod configuration;
mod page_processor;
mod structs;
mod validators;

#[tokio::main]
async fn main() {
    println!("====================================================================");
    println!("Welcome to");
    println!(
        r#"
    ___     _        __                                
    /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
   //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
  / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
  \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                       |_|     
                                                
    plese update the configuration.json file"#,
    );
    println!("====================================================================");

    let config: Config = Config::from_config_file("configuration/configuration.json").unwrap();
    let mut website = config.website.clone();
    if !website.starts_with("http://") {
        website = format!("http://{}", website);
    }
    let mut processing: HashSet<String> = HashSet::new();
    let mut processed: HashSet<String> = HashSet::new();
    let mut processed_resources: HashSet<String> = HashSet::new();
    processing.insert(website.to_string());
    println!(
        "initializing download directory [./{}]...",
        config.resources_directory
    );
    initialize_download_directory(&config);
    while processing.len() > 0 {
        let link = processing.clone();
        let link = link.iter().next().unwrap();
        println!("processing: {}", link);
        processing.remove(link.as_str());
        extract_links_and_process_data(
            link,
            &config,
            &mut processing,
            &mut processed,
            &mut processed_resources,
            &DomainFilter {
                is_same_domain: config.processing_same_domain,
                domain: config.website.to_owned(),
            },
            &mut ExtensionFilter {
                enabled: true,
                extensions: config.extensions.clone(),
                is_resource_same_domain: false,
            },
        )
        .await;
        processed.insert(link.to_string());
    }
}
