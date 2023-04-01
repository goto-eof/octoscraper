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
        config._is_image_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_IMAGE_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_IMAGE).is_some() {
            config._image_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_IMAGE)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_ENABLE_VIDEO_EXTRACTOR).is_some() {
        config._is_video_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_VIDEO_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_VIDEO).is_some() {
            config._video_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_VIDEO)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_ENABLE_AUDIO_EXTRACTOR).is_some() {
        config._is_audio_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_AUDIO_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_AUDIO).is_some() {
            config._audio_extractor_extensions = commands
                .get(ARGUMENT_EXTENSIONS_AUDIO)
                .unwrap()
                .split(",")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>();
        }
    }

    if commands.get(ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR).is_some() {
        config._is_other_extractor_enabled = commands
            .get(ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR)
            .unwrap()
            .parse()
            .unwrap();

        if commands.get(ARGUMENT_EXTENSIONS_OTHER_FILE).is_some() {
            config._other_extractor_extensions = commands
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

    if !config._is_audio_extractor_enabled
        && !config._is_image_extractor_enabled
        && !config._is_video_extractor_enabled
        && !config._is_other_extractor_enabled
    {
        println!("No job selected. Please select at least one job: image extraction, video extraction, audio extraction");
        return Flow::EXIT;
    }

    return Flow::CONTINUE;
}

fn print_help() {
    println!("                               Help");
    println!("====================================================================");
    println!("{}	website - without http and www prefix", ARGUMENT_WEBSITE);
    println!(
        "{}   enable image extractor",
        ARGUMENT_ENABLE_IMAGE_EXTRACTOR
    );
    println!(
        "{}   enable video extractor",
        ARGUMENT_ENABLE_VIDEO_EXTRACTOR
    );
    println!(
        "{}   enable audio extractor",
        ARGUMENT_ENABLE_AUDIO_EXTRACTOR
    );
    println!(
        "{}   enable other file extractor",
        ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR
    );
    println!(
        "{}	list of image extensions separated by comma",
        ARGUMENT_EXTENSIONS_IMAGE
    );
    println!(
        "{}	list of video extensions separated by comma",
        ARGUMENT_EXTENSIONS_VIDEO
    );
    println!(
        "{}	list of audio extensions separated by comma",
        ARGUMENT_EXTENSIONS_AUDIO
    );
    println!(
        "{}	list of other file extensions separated by comma",
        ARGUMENT_EXTENSIONS_OTHER_FILE
    );

    println!(
        "{} minimum image size (in bytes)",
        ARGUMENT_MINIMUM_SIZE_IMAGE
    );
    println!(
        "{} minimum audio size (in bytes)",
        ARGUMENT_MINIMUM_SIZE_AUDIO
    );
    println!(
        "{} minimum video size (in bytes)",
        ARGUMENT_MINIMUM_SIZE_VIDEO
    );
    println!(
        "{} minimum other file size (in bytes)",
        ARGUMENT_MINIMUM_SIZE_OTHER_FILE
    );

    println!(
        "{}	directory where files will be saved",
        ARGUMENT_RESOURCE_DIRECTORY
    );
    println!(
        "{}	sleep time in millis before making the request",
        ARGUMENT_SLEEP_TIME
    );
    println!("{}	download timeout", ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT);
    println!(
        "{}	insistent mode (it retries until download succeed)",
        ARGUMENT_INSISTENT_MODE
    );
    println!(
        "{}	download limit (by default it makes as much requests as possibile)",
        ARGUMENT_DOWNLOAD_LIMIT
    );
    println!("{}	user agent", ARGUMENT_USER_AGENT);
    println!(
        "{}	hash check: avoid duplicate downloads",
        ARGUMENT_HASH_CHECK
    );
    println!("{}    same domain", ARGUMENT_SAME_DOMAIN);
    println!("{} process only the root link", ARGUMENT_PROCESS_ONLY_ROOT);
    println!("{}    for this help message", ARGUMENT_HELP);
    println!("====================================================================");
}

#[derive(PartialEq)]
enum Flow {
    CONTINUE,
    EXIT,
}
