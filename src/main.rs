use config_file::FromConfigFile;

use crate::{
    configuration::Config,
    page_processor::{extract_links_and_process_data, DomainFilter, ExtensionFilter},
};
use std::collections::HashSet;

mod configuration;
mod page_processor;

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
    "#,
    );
    println!("====================================================================");

    let config = Config::from_config_file("configuration/configuration.json").unwrap();
    let website = config.website;
    let mut processing: HashSet<String> = HashSet::new();
    let mut processed: HashSet<String> = HashSet::new();
    processing.insert(website.to_string());
    while processing.len() > 0 {
        let link = processing.clone();
        let link = link.iter().next().unwrap();
        println!("processing: {}", link);
        processing.remove(link.as_str());
        extract_links_and_process_data(
            link,
            &mut processing,
            &mut processed,
            DomainFilter {
                is_same_domain: true,
                domain: website.to_string(),
            },
            &mut ExtensionFilter {
                enabled: true,
                extensions: vec![".png".to_owned(), ".jpg".to_owned(), ".jpeg".to_owned()],
            },
        )
        .await;
        processed.insert(link.to_string());
    }
}
