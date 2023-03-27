use serde::Deserialize;
pub struct DomainFilter {
    pub is_same_domain: bool,
    pub domain: String,
}
pub struct ExtensionFilter {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_resource_same_domain: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub website: String,
    pub processing_same_domain: bool,
    pub extensions: Vec<String>,
    pub resources_directory: String,
    pub sleep_time: u64,
    pub resource_download_timeout: u64,
}
