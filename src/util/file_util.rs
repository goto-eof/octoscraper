use crate::structure::application_settings::ApplicationSettings;
use crate::structure::config_struct::Config;
use rand::{distributions::Uniform, prelude::Distribution};
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::io;
use std::{fs, path::Path};

pub fn initialize_download_directory(config: &Config) -> () {
    let resources_directory = format!("./{}", config.resources_directory);
    if !Path::new(&resources_directory).is_dir() {
        fs::create_dir(&resources_directory).unwrap();
    }
}

pub fn generate_file_name(
    application_settings: &ApplicationSettings,
    extension_opt: Option<String>,
) -> String {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..10000000);
    let rndd = die.sample(&mut rng);
    let mut extension = application_settings.file_extension.to_owned();
    if extension_opt.is_some() {
        extension = extension_opt.unwrap();
    }
    let bkname = format!("random-{}.{}", rndd, extension);
    bkname
}

pub fn file_rename(config: &Config, path: &str, file_name: &str) {
    let mut new_file_name = format!("{}/{}", &config.resources_directory, file_name);
    let mut i = 1;
    while std::path::Path::new(&new_file_name).exists() {
        let tmp_fname = format!("{}-{}", extract_filename_prefix(file_name), i);
        let tmp_ext = extract_extension(file_name);
        new_file_name = format!("{}/{}.{}", &config.resources_directory, tmp_fname, tmp_ext);
        i = i + 1;
    }
    fs::rename(path, new_file_name).unwrap();
}

pub fn file_delete(path: &str) {
    fs::remove_file(path).unwrap();
}

pub fn file_len_less_than(file_name: &str, size: u64) -> bool {
    fs::metadata(file_name).unwrap().len() < size
}

pub fn extract_extension(filename: &str) -> String {
    return Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_string();
}

pub fn extract_filename_prefix(filename: &str) -> String {
    Path::new(filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn file_already_exists_with_same_hash(ready_file: &str, processing_file: &str) -> bool {
    return std::path::Path::new(&ready_file).exists()
        && calculate_file_hash(ready_file).eq(&calculate_file_hash(processing_file));
}

pub fn calculate_file_hash(path_and_fname: &str) -> String {
    let mut file = fs::File::open(&path_and_fname).unwrap();
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();
    let result = hex::encode(hash);
    return result;
}

pub fn calculate_file_path(config: &Config, fname: &str) -> String {
    format!("{}/{}", config.resources_directory, fname)
}
