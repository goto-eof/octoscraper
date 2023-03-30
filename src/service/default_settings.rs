use crate::structure::config_struct::Config;

pub fn load_default_settings() -> Config {
    return Config {
        website: "".to_owned(),
        processing_same_domain: true,
        extensions: vec![
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
        resources_directory: "resources".to_string(),
        sleep_time: 0,
        resource_download_timeout: 90000,
        insistent_mode: true,
        download_limit: 999999,
        user_agent: "".to_string(),
        hash_check: true,
    };
}
