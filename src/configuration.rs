use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub website: String,
    pub extensions: Vec<String>,
}
