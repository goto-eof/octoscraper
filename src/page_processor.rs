use rand::distributions::Uniform;
use rand::prelude::Distribution;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub async fn extract_links_and_process_data(
    link: &str,
    processing: &mut HashSet<String>,
    processed: &mut HashSet<String>,
    processed_resources: &mut HashSet<String>,
    domain_filter: &DomainFilter,
    extension_filter: &mut ExtensionFilter,
) {
    let response_str = reqwest::get(link).await.unwrap().text().await.unwrap();

    let extracted_links = extract_links(&response_str, &domain_filter).await;
    extracted_links.iter().for_each(|item| {
        let item = &link_normalizer(item);
        if !processed.contains(item.as_str()) {
            processing.insert(item.to_string());
        }
    });

    let resources_links = extract_resources(&response_str, &domain_filter, &extension_filter).await;
    for ele in resources_links.iter() {
        let ele = &link_normalizer(ele);
        if !processed_resources.contains(ele) {
            download(ele).await;
            processed_resources.insert(ele.to_owned());
        }
    }
}

pub fn link_normalizer(link: &str) -> String {
    let mut link = link.to_owned();
    if !link.starts_with("http:") {
        link = format!("http:{}", link);
    }
    return link;
}

async fn download(link: &str) {
    println!("downloading...: {}", link);
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..100000);
    let rndd = die.sample(&mut rng);
    let bkname = format!("random-{}.png", rndd);

    let image_file = reqwest::get(link).await.unwrap();
    let url = image_file.url().clone();
    let fname = url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or(bkname.as_ref());
    let image_file = image_file.bytes().await.unwrap();
    let image_file = &image_file.to_vec();
    if !Path::new("./images").is_dir() {
        fs::create_dir("./images").unwrap();
    }
    let fname = format!("./images/{}", fname);
    let mut file = File::create(fname).unwrap();
    file.write_all(image_file).unwrap();
}

pub async fn extract_links(response_str: &str, domain_filter: &DomainFilter) -> Vec<String> {
    if response_str.contains("<html") {
        return Document::from(response_str)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|item| item.to_string())
            .filter_map(|link| is_same_domain(&domain_filter.domain, &link))
            .collect();
    }
    return vec![];
}

pub async fn extract_resources(
    resource_str: &str,
    domain_filter: &DomainFilter,
    extension_filter: &ExtensionFilter,
) -> Vec<String> {
    return Document::from(resource_str)
        .find(Name("img"))
        .filter_map(|n| n.attr("src"))
        .map(|item| item.to_string())
        .filter_map(|link| is_same_domain(&domain_filter.domain, &link))
        .filter_map(|link| contains_extension(extension_filter.extensions.clone(), &link))
        .collect();
}

fn is_same_domain(domain: &str, link: &str) -> Option<String> {
    return if link.contains(domain) {
        Some(link.to_string())
    } else {
        None
    };
}
fn contains_extension(extensions: Vec<String>, link: &str) -> Option<String> {
    for extension in extensions {
        if link.ends_with(&extension) {
            return Some(link.to_string());
        }
    }
    return None;
}

pub struct DomainFilter {
    pub is_same_domain: bool,
    pub domain: String,
}
pub struct ExtensionFilter {
    pub enabled: bool,
    pub extensions: Vec<String>,
}
