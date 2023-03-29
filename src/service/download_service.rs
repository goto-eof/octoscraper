use crate::structure;
use rand::{distributions::Uniform, prelude::Distribution};
use std::{
    cmp::min,
    fs::File,
    io::Write,
    thread::{self},
    time,
};
use structure::config_struct::Config;
use tokio::{spawn, task::JoinHandle};

// TODO double check return statement, so that also refactor this method
pub async fn download(link: &str, config: &Config) -> Option<JoinHandle<(String, bool)>> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..100000);
    let rndd = die.sample(&mut rng);
    let bkname = format!("random-{}.png", rndd);
    let millis = time::Duration::from_millis(config.sleep_time);
    log::debug!("    sleeping {:?}....", millis);
    thread::sleep(millis);
    log::debug!("    retrieving image...");
    let client = reqwest::Client::builder()
        .user_agent("<<--[ OctoScraper ]-->>")
        .timeout(time::Duration::from_millis(
            config.resource_download_timeout,
        ))
        .build()
        .unwrap();
    let image_file = client.get(link).send().await;
    if image_file.is_ok() {
        let mut image_file = image_file.unwrap();
        let total_size = image_file.content_length();
        if total_size.is_none() {
            return None;
        }
        let total_size = total_size.unwrap();
        log::debug!("    image retrieved");
        let url = image_file.url().clone();
        let fname = url
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or(bkname.as_ref());
        let resources_directory = format!("./{}", config.resources_directory);
        let fname = format!("{}/{}", &resources_directory, fname);
        log::debug!("    creating empty file...");
        let mut file = File::create(&fname).unwrap();
        log::debug!("    created file");
        log::debug!("    writing image on file...");
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
        println!("    error while downloading image {}", link);
        return None;
    }
}
