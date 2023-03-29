use crate::structure;
use std::{
    cmp::min,
    fs::File,
    io::Write,
    thread::{self},
    time,
};
use structure::config_struct::Config;
use tokio::{spawn, task::JoinHandle};

use super::{file_service::generate_file_name, link_service::extract_fname};

pub async fn download(link: &str, config: &Config) -> Option<JoinHandle<(String, bool)>> {
    sleep(config);
    let image_file = retrieve_resource(config, link).await;
    if image_file.is_ok() {
        let mut image_file = image_file.unwrap();
        let total_size_opt = image_file.content_length();
        let mut total_size = 0;
        if total_size_opt.is_some() {
            total_size = total_size_opt.unwrap();
        }
        let alternative_file_name = generate_file_name();
        let file_name = extract_fname(link, Some(alternative_file_name));
        let resources_directory = format!("./{}", config.resources_directory);
        let full_path = format!("{}/{}", &resources_directory, file_name);
        let mut file = File::create(&full_path).unwrap();
        let mut downloaded: u64 = 0;
        let link = link.to_owned();
        let handler: JoinHandle<(String, bool)> = spawn(async move {
            while let Ok(item) = image_file.chunk().await {
                if item.is_some() {
                    let chunk = item.unwrap();
                    if file.write_all(&chunk).is_ok() {
                        downloaded = min(downloaded + (chunk.len() as u64), total_size);
                    } else {
                        return (link.to_owned(), false);
                    }
                } else {
                    return (link.to_owned(), true);
                }
            }
            return (link.to_owned(), true);
        });
        return Some(handler);
    } else {
        println!("error while downloading image {}", link);
        return None;
    }
}

async fn retrieve_resource(
    config: &Config,
    link: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("<<--[ OctoScraper ]-->>")
        .timeout(time::Duration::from_millis(
            config.resource_download_timeout,
        ))
        .build()
        .unwrap();
    let image_file = client.get(link).send().await;
    image_file
}

fn sleep(config: &Config) {
    if config.sleep_time > 0 {
        let millis = time::Duration::from_millis(config.sleep_time);
        thread::sleep(millis);
    }
}
