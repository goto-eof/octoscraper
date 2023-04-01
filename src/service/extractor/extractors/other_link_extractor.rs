use std::collections::HashSet;

use super::resource_extractor::{strategy_a_common_extractor, ResourceExtractor};

pub struct OtherFileLinkExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
    pub processing_page_link: String,
}

impl ResourceExtractor for OtherFileLinkExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_name(&self) -> String {
        return OtherFileLinkExtractor::EXTRACTOR_NAME.to_string();
    }

    fn extract(&self, resource_str: &str) -> HashSet<String> {
        let mut links: Vec<String> = Vec::new();

        if self.enabled {
            self.strategy_a(resource_str)
                .iter()
                .for_each(|elem| links.push(elem.to_string()));
        }

        return HashSet::from_iter(links.iter().cloned());
    }
}

impl OtherFileLinkExtractor {
    pub const EXTRACTOR_NAME: &str = "other-link-extractor";

    fn strategy_a(&self, resource_str: &str) -> Vec<String> {
        return strategy_a_common_extractor(
            resource_str,
            self.extensions.clone(),
            &self.domain,
            self.is_same_domain_enabled,
            self.processing_page_link.to_owned(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_b_test_extract_from_video_tag() {
        let link_extractor = OtherFileLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_string(),
            extensions: vec![".zip".to_owned(), ".pdf".to_owned()],
        };
        let resource_str = r#"
                <div>
                    <a href="http://dodu.it/test/ciao.html">dodu.it</a>.
                    <a href="http://dodu.it/test/ciao.pdf">Download pdf</a>.
                    <a href="http://dodu.it/test/ciao.zip">Download zip</a>.
                    <a href="http://dodu.it/test/ciao-ciao.html">dodu.it</a>.
                    <a href="http://dodu.it/test/ciao-ciao.jsp">dodu.it</a>.
                    <a href="http://dodu.it/test/ciao-ciao.strange">dodu.it</a>.
                </div>
        "#;
        let result = link_extractor.extract(resource_str);
        assert_eq!(2, result.len());
        assert_eq!(true, result.get("http://dodu.it/test/ciao.pdf").is_some());
        assert_eq!(true, result.get("http://dodu.it/test/ciao.zip").is_some());
    }
}
