use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub website: String,
    pub processing_same_domain: bool,
    pub resources_directory: String,
    pub sleep_time: u64,
    pub resource_download_timeout: u64,
    pub insistent_mode: bool,
    pub download_limit: i32,
    pub user_agent: String,
    pub hash_check: bool,
    pub process_only_root: bool,

    pub is_image_extractor_enabled: bool,
    pub image_extractor_minimum_size: u64,
    pub image_extractor_extensions: Vec<String>,
    pub is_video_extractor_enabled: bool,
    pub video_extractor_minimum_size: u64,
    pub video_extractor_extensions: Vec<String>,
    pub is_audio_extractor_enabled: bool,
    pub audio_extractor_minimum_size: u64,
    pub audio_extractor_extensions: Vec<String>,
    pub is_other_extractor_enabled: bool,
    pub other_file_extractor_minimum_size: u64,
    pub other_extractor_extensions: Vec<String>,

    pub resource_unique_method: u8,
}
