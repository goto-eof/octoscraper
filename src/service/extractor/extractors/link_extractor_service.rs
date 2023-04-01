use select::{document::Document, node::Node, predicate::Name};

use crate::util::{
    link_util::{add_base_url_if_not_present, normalize_link_replace_spaces},
    validation_util::is_same_domain_ext,
};

use super::resource_extractor::ResourceExtractor;

pub struct LinkExtractor {
    pub enabled: bool,
    pub is_same_domain_enabled: bool,
    pub domain: String,
    pub processing_page_link: String,
}

impl ResourceExtractor for LinkExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn extract(&self, resource_str: &str) -> Vec<String> {
        if self.enabled {
            let document = Document::from(resource_str);
            if is_document_html_file(&document) {
                return document
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .map(|item| item.to_string())
                    .map(|link| {
                        add_base_url_if_not_present(&link, &self.domain, &self.processing_page_link)
                    })
                    .filter_map(|link| {
                        is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link)
                    })
                    .filter_map(|link| normalize_link_replace_spaces(&link))
                    .collect();
            }
            return vec![];
        }
        return vec![];
    }
}

pub fn is_document_html_file(document: &Document) -> bool {
    let is_html: Vec<Node> = document.find(Name("html")).collect();
    return is_html.len() == 1;
}
