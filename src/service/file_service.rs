use std::{ffi::OsStr, fs, path::Path};

use rand::{distributions::Uniform, prelude::Distribution};

use crate::structure::config_struct::Config;

pub fn initialize_download_directory(config: &Config) -> () {
    let resources_directory = format!("./{}", config.resources_directory);
    if !Path::new(&resources_directory).is_dir() {
        fs::create_dir(&resources_directory).unwrap();
    }
}

pub fn generate_file_name(extension_opt: Option<String>) -> String {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..10000000);
    let rndd = die.sample(&mut rng);
    let mut extension = "os".to_owned();
    if extension_opt.is_some() {
        extension = extension_opt.unwrap();
    }
    let bkname = format!("random-{}.{}", rndd, extension);
    bkname
}

pub fn extract_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_owned()
}

pub fn file_rename(config: &Config, path: &str, file_name: &str) {
    fs::rename(
        path,
        format!("{}/{}", &config.resources_directory, file_name),
    )
    .unwrap();
}

pub fn file_delete(path: &str) {
    fs::remove_file(path).unwrap();
}
