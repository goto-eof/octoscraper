use std::collections::HashSet;

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

    fn get_name(&self) -> String {
        return LinkExtractor::EXTRACTOR_NAME.to_string();
    }

    fn extract(&self, resource_str: &str) -> HashSet<String> {
        if self.enabled {
            let document = Document::from(resource_str);
            if is_document_html_file(&document) {
                let links: Vec<String> = document
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .map(|item| item.to_string())
                    .map(|link| {
                        let result = add_base_url_if_not_present(
                            &link,
                            &self.domain,
                            &self.processing_page_link,
                        );
                        println!("processed: {}", result);
                        return result;
                    })
                    .filter_map(|link| {
                        is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link)
                    })
                    .filter_map(|link| normalize_link_replace_spaces(&link))
                    .collect();
                return HashSet::from_iter(links.iter().cloned());
            }
        }
        return HashSet::new();
    }
}

impl LinkExtractor {
    pub const EXTRACTOR_NAME: &str = "link-extractor";
}

pub fn is_document_html_file(document: &Document) -> bool {
    let is_html: Vec<Node> = document.find(Name("html")).collect();
    return is_html.len() == 1;
}

// =========================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_b_test_extract_from_video_tag() {
        let link_extractor = LinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_string(),
        };
        let resource_str = r#"
                <div>
                    <a href="http://dodu.it/test/ciao.html">dodu.it</a>.
                    <a href="http://www.dodu.it/test/ciao.html">dodu.it</a>.
                    <a href="//dodu.it/test/ciao-ciao.html">dodu.it</a>.
                    <a href="/ciao-ciao-mondo.html">dodu.it</a>.
                    <a href="ciao-ciao-mondo-crudele.html">dodu.it</a>.
                    <a href="./ciao-ciao-mondo-crudelissimo.html">dodu.it</a>.
                </div>
        "#;
        let result = link_extractor.extract(resource_str);
        assert_eq!(6, result.len());
        assert_eq!(true, result.get("http://dodu.it/test/ciao.html").is_some());
        assert_eq!(
            true,
            result.get("http://www.dodu.it/test/ciao.html").is_some()
        );
        assert_eq!(
            true,
            result.get("http://dodu.it/test/ciao-ciao.html").is_some()
        );
        assert_eq!(
            true,
            result.get("http://dodu.it/ciao-ciao-mondo.html").is_some()
        );
        assert_eq!(
            true,
            result
                .get("http://dodu.it/test/ciao-ciao-mondo-crudele.html")
                .is_some()
        );
        assert_eq!(
            true,
            result
                .get("http://dodu.it/test/ciao-ciao-mondo-crudelissimo.html")
                .is_some()
        );
    }
}
