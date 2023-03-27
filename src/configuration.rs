use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub website: String,
    pub processing_same_domain: bool,
    pub extensions: Vec<String>,
    pub resources_directory: String,
}
