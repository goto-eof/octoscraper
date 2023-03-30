use super::file_service::generate_file_name;
use crate::structure;
use std::{
    cmp::min,
    fs::File,
    io::Write,
    path::Path,
    thread::{self},
    time,
};
use structure::config_struct::Config;
use tokio::{spawn, task::JoinHandle};

pub async fn download(link: &str, config: &Config) -> Option<JoinHandle<(String, bool, String)>> {
    sleep(config);
    let image_file = retrieve_resource(config, link).await;
    if image_file.is_ok() {
        let mut image_file = image_file.unwrap();
        let total_size_opt = image_file.content_length();
        let mut total_size = 0;
        if total_size_opt.is_some() {
            total_size = total_size_opt.unwrap();
        }

        let resources_directory = format!("./{}", config.resources_directory);
        let mut file_name = generate_file_name(None);
        let mut full_path = format!("{}/{}", &resources_directory, file_name);
        while Path::exists(Path::new(&full_path)) {
            file_name = generate_file_name(None);
            full_path = format!("{}/{}", &resources_directory, file_name);
        }

        let mut file = File::create(&full_path).unwrap();
        let mut downloaded: u64 = 0;
        let link = link.to_owned();
        let handler: JoinHandle<(String, bool, String)> = spawn(async move {
            while let Ok(item) = image_file.chunk().await {
                if item.is_some() {
                    let chunk = item.unwrap();
                    if file.write_all(&chunk).is_ok() {
                        downloaded = min(downloaded + (chunk.len() as u64), total_size);
                    } else {
                        return (link.to_owned(), false, full_path);
                    }
                } else {
                    return (link.to_owned(), true, full_path);
                }
            }
            return (link.to_owned(), true, full_path);
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
        .user_agent(calculate_user_agent(config))
        .timeout(time::Duration::from_millis(
            config.resource_download_timeout,
        ))
        .build()
        .unwrap();
    let image_file = client.get(link).send().await;
    image_file
}

fn calculate_user_agent(config: &Config) -> String {
    if config.user_agent.is_empty() {
        return format!(
            "OctoScraper v. {}",
            option_env!("CARGO_PKG_VERSION").unwrap()
        )
        .to_owned();
    }
    return config.user_agent.to_owned();
}

fn sleep(config: &Config) {
    if config.sleep_time > 0 {
        let millis = time::Duration::from_millis(config.sleep_time);
        thread::sleep(millis);
    }
}
