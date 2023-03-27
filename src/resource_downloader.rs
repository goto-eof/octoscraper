use std::{fs::File, io::Write, thread, time};

use rand::{distributions::Uniform, prelude::Distribution};

use crate::structs::Config;

pub async fn download(link: &str, config: &Config) -> bool {
    println!("    downloading [{}]:", link);
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..100000);
    let rndd = die.sample(&mut rng);
    let bkname = format!("random-{}.png", rndd);
    let millis = time::Duration::from_millis(config.sleep_time);
    println!("    sleeping {:?}....", millis);
    thread::sleep(millis);
    println!("    retrieving image...");
    let client = reqwest::Client::builder()
        .user_agent("<<--[ OctoScraper ]-->>")
        .timeout(time::Duration::from_millis(
            config.resource_download_timeout,
        ))
        .build()
        .unwrap();
    let image_file = client.get(link).send().await;
    if image_file.is_ok() {
        let image_file = image_file.unwrap();
        println!("    image retrieved");
        let url = image_file.url().clone();
        let fname = url
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or(bkname.as_ref());
        println!("    converting image to bytes...");
        let image_file = image_file.bytes().await;
        if image_file.is_ok() {
            let image_file = image_file.unwrap();
            println!("    converted image to bytes");
            let image_file = &image_file.to_vec();
            println!("    converted image to vec");
            let resources_directory = format!("./{}", config.resources_directory);
            let fname = format!("./{}/{}", &resources_directory, fname);
            println!("    creating file...");
            let mut file = File::create(fname).unwrap();
            println!("    created file");
            println!("    writing image on file...");
            file.write_all(image_file).unwrap();
            println!("    writed image on file.");
            return true;
        }
        println!(
            "    download timeout ({}). Retrying later.",
            config.resource_download_timeout
        );
        return false;
    } else {
        println!("    error while downloading image {}", link);
        return false;
    }
}
