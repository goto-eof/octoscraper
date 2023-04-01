use config_file::FromConfigFile;
use std::collections::{HashMap, HashSet};
use std::env;
use util::settings_util::load_default_settings;

use crate::service::page_processor_service::extract_links_and_process_data;
use crate::structure::application_settings::ApplicationSettings;
use crate::structure::config_struct::Config;
use crate::structure::processed_hash_struct::ProcessedHash;
use crate::structure::processed_struct::Processed;
use crate::util::file_util::initialize_download_directory;

mod service;
mod structure;
mod util;

const ARGUMENT_HELP: &str = "-h";
const ARGUMENT_WEBSITE: &str = "-w";
const ARGUMENT_EXTENSIONS_IMAGE: &str = "-ei";
const ARGUMENT_EXTENSIONS_VIDEO: &str = "-ev";
const ARGUMENT_EXTENSIONS_AUDIO: &str = "-ea";
const ARGUMENT_EXTENSIONS_OTHER_FILE: &str = "-eo";

const ARGUMENT_MINIMUM_SIZE_IMAGE: &str = "-si";
const ARGUMENT_MINIMUM_SIZE_VIDEO: &str = "-sv";
const ARGUMENT_MINIMUM_SIZE_AUDIO: &str = "-sa";
const ARGUMENT_MINIMUM_SIZE_OTHER_FILE: &str = "-so";

const ARGUMENT_ENABLE_IMAGE_EXTRACTOR: &str = "-oi";
const ARGUMENT_ENABLE_VIDEO_EXTRACTOR: &str = "-ov";
const ARGUMENT_ENABLE_AUDIO_EXTRACTOR: &str = "-oa";
const ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR: &str = "-oo";
const ARGUMENT_RESOURCE_DIRECTORY: &str = "-d";
const ARGUMENT_SLEEP_TIME: &str = "-s";
const ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT: &str = "-t";
const ARGUMENT_INSISTENT_MODE: &str = "-i";
const ARGUMENT_DOWNLOAD_LIMIT: &str = "-l";
const ARGUMENT_USER_AGENT: &str = "-a";
const ARGUMENT_HASH_CHECK: &str = "-c";
const ARGUMENT_SAME_DOMAIN: &str = "-sd";
const ARGUMENT_PROCESS_ONLY_ROOT: &str = "-r";

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
    initialize_download_directory(&config);
    while processing.len() > 0 {
        let link = processing.clone();
        let link = link.iter().next().unwrap();
        println!("\nprocessing: {}", link);
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
    println!("{} processd.\nExiting from applciation.", config.website);
}

fn update_config_with_argument_values(config: &mut Config) -> Flow {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_help();
        println!("No arguments were passed. Exiting...");
        return Flow::EXIT;
    }

    if args.get(1).unwrap().eq(ARGUMENT_HELP) {
        print_help();
        return Flow::EXIT;
    }

    if (args.len() - 1) % 2 == 1 {
        println!("Invalid number of arguments!");
        panic!("Exiting, because you provided an invalid number of arguments.")
    }

    let mut commands: HashMap<String, String> = HashMap::new();
    for i in 0..(args.len()) {
        if i % 2 == 1 {
            commands.insert(
                args.get(i).unwrap().to_string(),
                args.get(i + 1).unwrap().to_string(),
            );
        }
    }

    if commands.get(ARGUMENT_WEBSITE).is_some() {
        config.website = commands.get(ARGUMENT_WEBSITE).unwrap().to_string();
    } else {
        println!("No website target specified. Exiting...");
        return Flow::EXIT;
    }

    if commands.get(ARGUMENT_SAME_DOMAIN).is_some() {
        config.processing_same_domain =
            commands.get(ARGUMENT_SAME_DOMAIN).unwrap().parse().unwrap();
    }

    if commands.get(ARGUMENT_ENABLE_IMAGE_EXTRACTOR).is_some() {
        config.is_image_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_IMAGE_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_IMAGE).is_some() {
            config.image_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_IMAGE)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_ENABLE_VIDEO_EXTRACTOR).is_some() {
        config.is_video_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_VIDEO_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_VIDEO).is_some() {
            config.video_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_VIDEO)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_ENABLE_AUDIO_EXTRACTOR).is_some() {
        config.is_audio_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_AUDIO_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_AUDIO).is_some() {
            config.audio_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_AUDIO)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR).is_some() {
        config.is_other_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_OTHER_FILE).is_some() {
            config.other_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_OTHER_FILE)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_RESOURCE_DIRECTORY).is_some() {
        config.resources_directory = commands
            .get(ARGUMENT_RESOURCE_DIRECTORY)
            .unwrap()
            .to_owned();
    }

    if commands.get(ARGUMENT_SLEEP_TIME).is_some() {
        config.sleep_time = commands.get(ARGUMENT_SLEEP_TIME).unwrap().parse().unwrap();
    }

    if commands.get(ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT).is_some() {
        config.resource_download_timeout = commands
            .get(ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_INSISTENT_MODE).is_some() {
        config.insistent_mode = commands
            .get(ARGUMENT_INSISTENT_MODE)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_DOWNLOAD_LIMIT).is_some() {
        config.download_limit = commands
            .get(ARGUMENT_DOWNLOAD_LIMIT)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_USER_AGENT).is_some() {
        config.user_agent = commands.get(ARGUMENT_USER_AGENT).unwrap().to_owned();
    }

    if commands.get(ARGUMENT_HASH_CHECK).is_some() {
        config.hash_check = commands.get(ARGUMENT_HASH_CHECK).unwrap().parse().unwrap();
    }

    if commands.get(ARGUMENT_PROCESS_ONLY_ROOT).is_some() {
        config.process_only_root = commands
            .get(ARGUMENT_PROCESS_ONLY_ROOT)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_MINIMUM_SIZE_IMAGE).is_some() {
        config.image_extractor_minimum_size = commands
            .get(ARGUMENT_MINIMUM_SIZE_IMAGE)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_MINIMUM_SIZE_AUDIO).is_some() {
        config.audio_extractor_minimum_size = commands
            .get(ARGUMENT_MINIMUM_SIZE_AUDIO)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_MINIMUM_SIZE_VIDEO).is_some() {
        config.video_extractor_minimum_size = commands
            .get(ARGUMENT_MINIMUM_SIZE_VIDEO)
            .unwrap()
            .parse()
            .unwrap();
    }

    if commands.get(ARGUMENT_MINIMUM_SIZE_OTHER_FILE).is_some() {
        config.other_file_extractor_minimum_size = commands
            .get(ARGUMENT_MINIMUM_SIZE_OTHER_FILE)
            .unwrap()
            .parse()
            .unwrap();
    }

    if !config.is_audio_extractor_enabled
        && !config.is_image_extractor_enabled
        && !config.is_video_extractor_enabled
        && !config.is_other_extractor_enabled
    {
        println!("No job selected. Please select at least one job: image extraction, video extraction, audio extraction");
        return Flow::EXIT;
    }

    return Flow::CONTINUE;
}

fn check_and_insert(map: &mut Vec<(String, String)>, key: &str, value: &str) {
    let duplicates: Vec<(String, String)> = map
        .iter()
        .filter(|(vec_key, _)| vec_key.eq(key))
        .map(|item| item.to_owned())
        .collect();
    if duplicates.len() != 0 {
        panic!("FATAL ERROR: command '{}' already mapper.", key);
    }
    map.push((key.to_owned(), value.to_owned()));
}

fn print_help() {
    let mut help_map: Vec<(String, String)> = Vec::new();

    check_and_insert(
        &mut help_map,
        ARGUMENT_WEBSITE,
        "website - with http/https prefix",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_ENABLE_IMAGE_EXTRACTOR,
        "enable image extractor",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_ENABLE_VIDEO_EXTRACTOR,
        "enable video extractor",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_ENABLE_AUDIO_EXTRACTOR,
        "enable audio extractor",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR,
        "enable other file extractor",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_EXTENSIONS_IMAGE,
        "list of image extensions separated by comma",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_EXTENSIONS_VIDEO,
        "list of video extensions separated by comma",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_EXTENSIONS_AUDIO,
        "list of audio extensions separated by comma",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_EXTENSIONS_OTHER_FILE,
        "list of other file extensions separated by comma",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_MINIMUM_SIZE_IMAGE,
        "minimum image size (in bytes)",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_MINIMUM_SIZE_AUDIO,
        "minimum audio size (in bytes)",
    );
    check_and_insert(
        &mut help_map,
        ARGUMENT_MINIMUM_SIZE_VIDEO,
        "minimum video size (in bytes)",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_MINIMUM_SIZE_OTHER_FILE,
        "minimum other file size (in bytes)",
    );
    check_and_insert(
        &mut help_map,
        ARGUMENT_RESOURCE_DIRECTORY,
        "directory where files will be saved",
    );
    check_and_insert(
        &mut help_map,
        ARGUMENT_SLEEP_TIME,
        "sleep time in millis before making the request",
    );
    check_and_insert(
        &mut help_map,
        ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT,
        "download timeout",
    );

    check_and_insert(
        &mut help_map,
        ARGUMENT_INSISTENT_MODE,
        "insistent mode (it retries until download succeed)",
    );
    check_and_insert(
        &mut help_map,
        ARGUMENT_DOWNLOAD_LIMIT,
        "download limit (by default it makes as much requests as possibile)",
    );
    check_and_insert(&mut help_map, ARGUMENT_USER_AGENT, "user agent");
    check_and_insert(
        &mut help_map,
        ARGUMENT_HASH_CHECK,
        "hash check: avoid duplicate downloads",
    );

    check_and_insert(&mut help_map, ARGUMENT_SAME_DOMAIN, "same domain");
    check_and_insert(
        &mut help_map,
        ARGUMENT_PROCESS_ONLY_ROOT,
        "process only the root link",
    );
    check_and_insert(&mut help_map, ARGUMENT_HELP, "for this help message");

    println!("                               Help");
    println!("====================================================================");
    println!();
    help_map
        .iter()
        .for_each(|(key, value)| println!("{} {}", key, value));
    println!("====================================================================");
}

#[derive(PartialEq)]
enum Flow {
    CONTINUE,
    EXIT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn strategy_b_test_extract_from_video_tag() {
        let mut map: Vec<(String, String)> = Vec::new();
        check_and_insert(&mut map, "-ad", "value 1");
        check_and_insert(&mut map, "-ad", "value 2");
    }
}
