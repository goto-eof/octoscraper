use crate::structure::config_struct::Config;
use rand::{distributions::Uniform, prelude::Distribution};
use std::ffi::OsStr;
use std::{fs, path::Path};

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
