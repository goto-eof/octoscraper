use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub website: String,
    pub processing_same_domain: bool,
    pub extensions: Vec<String>,
    pub resources_directory: String,
    pub sleep_time: u64,
    pub resource_download_timeout: u64,
    pub insistent_mode: bool,
    pub download_limit: i32,
    pub user_agent: String,
    pub hash_check: bool,
}
