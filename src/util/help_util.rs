pub const ARGUMENT_HELP: &str = "-h";
pub const ARGUMENT_WEBSITE: &str = "-w";
pub const ARGUMENT_EXTENSIONS_IMAGE: &str = "-ei";
pub const ARGUMENT_EXTENSIONS_VIDEO: &str = "-ev";
pub const ARGUMENT_EXTENSIONS_AUDIO: &str = "-ea";
pub const ARGUMENT_EXTENSIONS_OTHER_FILE: &str = "-eo";

pub const ARGUMENT_MINIMUM_SIZE_IMAGE: &str = "-si";
pub const ARGUMENT_MINIMUM_SIZE_VIDEO: &str = "-sv";
pub const ARGUMENT_MINIMUM_SIZE_AUDIO: &str = "-sa";
pub const ARGUMENT_MINIMUM_SIZE_OTHER_FILE: &str = "-so";
pub const ARGUMENT_ENABLE_IMAGE_EXTRACTOR: &str = "-oi";
pub const ARGUMENT_ENABLE_VIDEO_EXTRACTOR: &str = "-ov";
pub const ARGUMENT_ENABLE_AUDIO_EXTRACTOR: &str = "-oa";
pub const ARGUMENT_ENABLE_OTHER_FILE_EXTRACTOR: &str = "-oo";
pub const ARGUMENT_RESOURCE_DIRECTORY: &str = "-d";
pub const ARGUMENT_SLEEP_TIME: &str = "-s";
pub const ARGUMENT_RESOURCE_DOWNLOAD_TIMEOUT: &str = "-t";
pub const ARGUMENT_INSISTENT_MODE: &str = "-i";
pub const ARGUMENT_DOWNLOAD_LIMIT: &str = "-l";
pub const ARGUMENT_USER_AGENT: &str = "-a";
pub const ARGUMENT_HASH_CHECK: &str = "-c";
pub const ARGUMENT_SAME_DOMAIN: &str = "-sd";
pub const ARGUMENT_PROCESS_ONLY_ROOT: &str = "-r";
pub const ARGUMENT_RESOURCE_PROCESS_UNIQUE_METHOD: &str = "-u";

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

pub fn print_help() {
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
        ARGUMENT_RESOURCE_PROCESS_UNIQUE_METHOD,
        "consider unique resources by filename (1) or by link (2). Allowed valued: 1 or 2",
    );
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
pub enum Flow {
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
