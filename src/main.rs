use crate::service::page_processor_service::extract_links_and_process_data;
use crate::structure::application_settings::ApplicationSettings;
use crate::structure::config_struct::Config;
use crate::structure::processed_hash_struct::ProcessedHash;
use crate::structure::processed_struct::Processed;
use crate::util::env_util::update_config_with_argument_values;
use crate::util::file_util::initialize_download_directory;
use crate::util::help_util::Flow;
use config_file::FromConfigFile;
use std::collections::HashSet;
use util::settings_util::load_default_settings;
mod service;
mod structure;
mod util;

#[tokio::main]
async fn main() {
    println!("====================================================================");
    println!("Welcome to");
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    println!(
        r#"
    ___     _        __                                
    /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
   //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
  / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
  \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                       |_|                (v. {})
   -h for help                                             
  "#,
        VERSION.unwrap()
    );
    println!("====================================================================");
    let application_settings_res =
        ApplicationSettings::from_config_file("configuration/configuration.json");
    let mut application_settings = ApplicationSettings {
        file_extension: "os".to_owned(),
    };
    if application_settings_res.is_ok() {
        application_settings = application_settings_res.ok().unwrap();
    }
    let mut config: Config = load_default_settings();

    if update_config_with_argument_values(&mut config) == Flow::EXIT {
        return;
    }
    let mut processing: HashSet<String> = HashSet::new();
    let mut processed: HashSet<String> = HashSet::new();
    let mut processed_resources: Processed = Processed::new();
    let mut processed_resources_hash: ProcessedHash = ProcessedHash::new(config.hash_check);
    processing.insert(config.website.to_string());
    println!(
        "initializing download directory [./{}]...",
        config.resources_directory
    );
    initialize_download_directory(&config, &application_settings);
    while processing.len() > 0 {
        let link = processing.clone();
        let link = link.iter().next().unwrap();
        println!(
            "\n--------------[ processing ]--------------------
            \n {}
            \n",
            link
        );
        processing.remove(link.as_str());
        let result = extract_links_and_process_data(
            &application_settings,
            link,
            &config,
            &mut processing,
            &mut processed,
            &mut processed_resources,
            &mut processed_resources_hash,
        )
        .await;
        let exit_status = result.0;
        let exit_message = result.1;
        processed.insert(link.to_string());
        if !exit_status {
            println!("ERROR: {}", exit_message);
        }
    }
    println!("{} processd.\nExiting from application.", config.website);
}
