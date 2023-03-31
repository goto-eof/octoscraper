use service::default_settings::load_default_settings;
use std::collections::{HashMap, HashSet};
use std::env;

use crate::service::file_service::initialize_download_directory;
use crate::service::page_processor_service::extract_links_and_process_data;
use crate::structure::config_struct::Config;
use crate::structure::processed_hash_struct::ProcessedHash;
use crate::structure::processed_struct::Processed;

mod service;
mod structure;

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

    env_logger::init();
    let mut config: Config = load_default_settings();

    if update_config_with_argument_values(&mut config) == Flow::EXIT {
        return;
    }
    let mut website = config.website.clone();
    if !website.starts_with("http://") {
        website = format!("http://{}", website);
    }
    let mut processing: HashSet<String> = HashSet::new();
    let mut processed: HashSet<String> = HashSet::new();
    let mut processed_resources: Processed = Processed::new();
    let mut processed_resources_hash: ProcessedHash = ProcessedHash::new(config.hash_check);
    processing.insert(website.to_string());
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
        extract_links_and_process_data(
            link,
            &config,
            &mut processing,
            &mut processed,
            &mut processed_resources,
            &mut processed_resources_hash,
        )
        .await;
        processed.insert(link.to_string());
    }
}

fn update_config_with_argument_values(config: &mut Config) -> Flow {
    const ARGUMENT_HELP: &str = "-h";
    const ARGUMENT_WEBSITE: &str = "-w";
    const ARGUMENT_EXTENSIONS_IMAGE: &str = "-ei";
    const ARGUMENT_EXTENSIONS_VIDEO: &str = "-ev";
    const ARGUMENT_EXTENSIONS_AUDIO: &str = "-ea";
    const ARGUMENT_ENABLE_IMAGE_EXTRACTOR: &str = "-oi";
    const ARGUMENT_ENABLE_VIDEO_EXTRACTOR: &str = "-ov";
    const ARGUMENT_ENABLE_AUDIO_EXTRACTOR: &str = "-oa";
    const ARGUMENT_RESOURCE_DIRECTORY: &str = "-d";
    const ARGUMENT_SLEEP_TIME: &str = "-s";
    const ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT: &str = "-t";
    const ARGUMENT_INSISTENT_MODE: &str = "-i";
    const ARGUMENT_DOWNLOAD_LIMIT: &str = "-l";
    const ARGUMENT_USER_AGENT: &str = "-a";
    const ARGUMENT_HASH_CHECK: &str = "-c";

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

    return Flow::CONTINUE;
}

fn print_help() {
    println!("                               Help");
    println!("====================================================================");
    println!("-w	website - without http and www prefix");
    println!("-oi   enable image extractor");
    println!("-ov   enable video extractor");
    println!("-oa   enable audio extractor");
    println!("-ei	list of image extensions separated by comma");
    println!("-ev	list of video extensions separated by comma");
    println!("-ea	list of audio extensions separated by comma");
    println!("-d	directory where files will be saved");
    println!("-s	sleep time in millis before making the request");
    println!("-t	download timeout");
    println!("-i	insistent mode (it retries until download succeed)");
    println!("-l	download limit (by default it makes as much requests as possibile)");
    println!("-a	user agent");
    println!("-c	hash check: avoid duplicate downloads");
    println!("-h    for this help message");
    println!("====================================================================");
}

#[derive(PartialEq)]
enum Flow {
    CONTINUE,
    EXIT,
}
