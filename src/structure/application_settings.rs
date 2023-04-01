use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub file_extension: String,
}
