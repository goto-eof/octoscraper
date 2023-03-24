use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;

pub async fn extract_links_and_process_data(
    link: &str,
    domain: &str,
    processing: &mut HashSet<String>,
    processed: &mut HashSet<String>,
) {
    println!("processing: {}", link);
    // retrieve all page links
    let extracted_links = extract_links(link).await;
    // save results in the map
    let filtered_links = apply_filters(
        extracted_links.clone(),
        DomainFilter {
            is_same_domain: true,
            domain: domain.to_string(),
        },
        ExtensionFilter {
            enabled: false,
            extensions: vec![],
        },
    );
    filtered_links.iter().for_each(|item| {
        if !processed.contains(item.as_str()) {
            processing.insert(item.to_string());
            processed.insert(item.to_string());
        }
    });
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
fn contains_extension(extensions: Vec<String>, link: &str) -> Option<String> {
    for extension in extensions {
        if link.ends_with(&extension) {
            return Some(link.to_string());
        }
    }
    return None;
}

fn apply_filters(
    links: Vec<String>,
    domain_filter: DomainFilter,
    extension_filter: ExtensionFilter,
) -> Vec<String> {
    let mut filtered_links: Vec<String> = links;
    if domain_filter.is_same_domain {
        filtered_links = filtered_links
            .iter()
            .filter_map(|link| is_same_domain(&domain_filter.domain, link))
            .collect();
    }
    if extension_filter.enabled {
        filtered_links = filtered_links
            .iter()
            .filter_map(|link| contains_extension(extension_filter.extensions.clone(), link))
            .collect();
    }
    return filtered_links;
}

struct DomainFilter {
    is_same_domain: bool,
    domain: String,
}
struct ExtensionFilter {
    enabled: bool,
    extensions: Vec<String>,
}
