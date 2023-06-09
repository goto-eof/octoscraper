use crate::structure::config_struct::Config;

pub fn load_default_settings() -> Config {
    return Config {
        website: "".to_owned(),
        processing_same_domain: true,
        process_only_root: false,
        resources_directory: "resources".to_string(),
        sleep_time: 0,
        resource_download_timeout: 90000,
        insistent_mode: true,
        download_limit: 999999,
        user_agent: "".to_string(),
        hash_check: true,

        is_image_extractor_enabled: false,
        image_extractor_extensions: vec![
            ".jpg".to_string(),
            ".jpeg".to_string(),
            ".png".to_string(),
            ".JPG".to_string(),
            ".JPEG".to_string(),
            ".PNG".to_string(),
            ".bmp".to_string(),
            ".BMP".to_string(),
            ".svg".to_string(),
            ".SVG".to_string(),
        ],
        is_video_extractor_enabled: false,
        video_extractor_extensions: vec![
            ".mp4".to_string(),
            ".MP4".to_string(),
            ".ogg".to_string(),
            ".OGG".to_string(),
        ],
        is_audio_extractor_enabled: false,
        audio_extractor_extensions: vec![
            ".mp3".to_string(),
            ".MP3".to_string(),
            ".mid".to_string(),
            ".MID".to_string(),
        ],
        is_other_extractor_enabled: false,
        other_extractor_extensions: vec![
            ".exe".to_string(),
            ".zip".to_string(),
            ".gz".to_string(),
            ".tar".to_string(),
        ],
        image_extractor_minimum_size: u64::MIN,
        video_extractor_minimum_size: u64::MIN,
        audio_extractor_minimum_size: u64::MIN,
        other_file_extractor_minimum_size: u64::MIN,
        resource_unique_method: 1,
    };
}
