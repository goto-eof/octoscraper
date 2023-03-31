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

    pub _is_image_extractor_enabled: bool,
    pub _image_extractor_extensions: Vec<String>,
    pub _is_video_extractor_enabled: bool,
    pub _video_extractor_extensions: Vec<String>,
    pub _is_audio_extractor_enabled: bool,
    pub _audio_extractor_extensions: Vec<String>,
}
