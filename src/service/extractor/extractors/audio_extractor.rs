use select::{document::Document, predicate::Name};

use crate::service::{
    link_service::{normalize_link_replace_spaces, normalize_src},
    validation_service::is_same_domain_ext,
};

use super::resource_extractor::ResourceExtractor;

pub struct AudioExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
}
// TODO
impl ResourceExtractor for AudioExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn extract(&self, resource_str: &str) -> Vec<String> {
        if self.enabled {
            let mut audio1: Vec<String> = Vec::new();

            let audio2: Vec<String> = Document::from(resource_str)
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .map(|item| item.to_string())
                .filter(|link| {
                    println!("link: {}", link);
                    for extension in self.extensions.iter() {
                        if link.ends_with(extension) {
                            return true;
                        }
                    }
                    return false;
                })
                .map(|link| normalize_link_replace_spaces(&link))
                .map(|link| normalize_src(&link, &self.domain))
                .filter_map(|link| {
                    is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link)
                })
                .collect();

            audio2.iter().for_each(|elem| audio1.push(elem.to_string()));
            return audio1;
        }
        return vec![];
    }
}
