use rand::distributions::Uniform;
use rand::prelude::Distribution;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

pub async fn extract_links_and_process_data(
    link: &str,
    domain: &str,
    processing: &mut HashSet<String>,
    processed: &mut HashSet<String>,
    domain_filter: DomainFilter,
    extension_filter: &mut ExtensionFilter,
) {
    println!("processing: {}", link);
    // retrieve all page links
    let extracted_links = extract_links(link).await;
    // first links filter
    let filtered_links = apply_filters(extracted_links.clone(), &domain_filter, None);
    // save results in the map
    filtered_links.iter().for_each(|item| {
        if !processed.contains(item.as_str()) {
            processing.insert(item.to_string());
        }
    });

    let filtered_links = apply_filters(
        extracted_links.clone(),
        &domain_filter,
        Some(&extension_filter),
    );
    for ele in filtered_links.iter() {
        println!("Saving: {}", ele);
        download(ele).await;
    }
}

async fn download(link: &str) {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..100000);
    let rndd = die.sample(&mut rng);
    let bkname = format!("random-{}.png", rndd);

    let image_file = reqwest::get(link).await.unwrap();

    let fname = image_file
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or(bkname.as_ref());
    let image_file = reqwest::get(link).await.unwrap();
    let image_file = image_file.bytes().await.unwrap();
    let image_file = &image_file.to_vec();

    let mut file = File::create(fname).unwrap();
    file.write_all(image_file).unwrap();
}

pub async fn extract_links(link: &str) -> Vec<String> {
    let res = reqwest::get(link).await.unwrap().text().await.unwrap();
    return Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|item| item.to_string())
        .collect();
}

fn is_same_domain(domain: &str, link: &str) -> Option<String> {
    return if link.starts_with(domain) {
        Some(link.to_string())
    } else {
        None
    };
}
fn contains_extension(extensions: Vec<String>, link: &str, contains: bool) -> Option<String> {
    for extension in extensions {
        let check: bool = link.ends_with(&extension);
        if check {
            return Some(link.to_string());
        }
    }
    return None;
}

fn apply_filters(
    links: Vec<String>,
    domain_filter: &DomainFilter,
    extension_filter: Option<&ExtensionFilter>,
) -> Vec<String> {
    let mut filtered_links: Vec<String> = links;
    if domain_filter.is_same_domain {
        filtered_links = filtered_links
            .iter()
            .filter_map(|link| is_same_domain(&domain_filter.domain, link))
            .collect();
    }

    if extension_filter.is_some() {
        let extension_filter = extension_filter.unwrap();
        if extension_filter.enabled {
            filtered_links = filtered_links
                .iter()
                .filter_map(|link| {
                    contains_extension(extension_filter.extensions.clone(), link, false)
                })
                .collect();
        }
    }
    return filtered_links;
}

pub struct DomainFilter {
    pub is_same_domain: bool,
    pub domain: String,
}
pub struct ExtensionFilter {
    pub enabled: bool,
    pub extensions: Vec<String>,
}
