use super::help_util::{
    ARGUMENT_DOWNLOAD_LIMIT, ARGUMENT_ENABLE_AUDIO_EXTRACTOR, ARGUMENT_ENABLE_IMAGE_EXTRACTOR,
    ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR, ARGUMENT_ENABLE_VIDEO_EXTRACTOR,
    ARGUMENT_EXTENSIONS_AUDIO, ARGUMENT_EXTENSIONS_IMAGE, ARGUMENT_EXTENSIONS_OTHER_FILE,
    ARGUMENT_EXTENSIONS_VIDEO, ARGUMENT_HASH_CHECK, ARGUMENT_HELP, ARGUMENT_INSISTENT_MODE,
    ARGUMENT_MINIMUM_SIZE_AUDIO, ARGUMENT_MINIMUM_SIZE_IMAGE, ARGUMENT_MINIMUM_SIZE_OTHER_FILE,
    ARGUMENT_MINIMUM_SIZE_VIDEO, ARGUMENT_PROCESS_ONLY_ROOT, ARGUMENT_RESOURCE_DIRECTORY,
    ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT, ARGUMENT_RESOURCE_PROCESS_UNIQUE_METHOD,
    ARGUMENT_SAME_DOMAIN, ARGUMENT_SLEEP_TIME, ARGUMENT_USER_AGENT, ARGUMENT_WEBSITE,
};
use crate::{
    structure::config_struct::Config,
    util::help_util::{print_help, Flow},
};
use std::{collections::HashMap, env};

pub fn update_config_with_argument_values(config: &mut Config) -> Flow {
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

    if commands
        .get(ARGUMENT_RESOURCE_PROCESS_UNIQUE_METHOD)
        .is_some()
    {
        config.resource_unique_method = commands
            .get(ARGUMENT_RESOURCE_PROCESS_UNIQUE_METHOD)
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
