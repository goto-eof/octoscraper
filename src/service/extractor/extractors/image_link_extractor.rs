use std::collections::HashSet;

use crate::util::{
    link_util::add_base_url_if_not_present,
    validation_util::{contains_extension, is_same_domain_ext},
};
use select::{document::Document, predicate::Name};

use super::resource_extractor::{strategy_a_common_extractor, ResourceExtractor};

pub struct ImageLinkExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
    pub processing_page_link: String,
}

impl ResourceExtractor for ImageLinkExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_name(&self) -> String {
        return ImageLinkExtractor::EXTRACTOR_NAME.to_string();
    }

    fn extract(&self, resource_str: &str) -> HashSet<String> {
        let mut links: Vec<String> = Vec::new();

        if self.enabled {
            self.strategy_a(resource_str)
                .iter()
                .for_each(|elem| links.push(elem.to_string()));
            self.strategy_b(resource_str)
                .iter()
                .for_each(|elem| links.push(elem.to_string()));
        }

        return HashSet::from_iter(links.iter().cloned());
    }
}

impl ImageLinkExtractor {
    pub const EXTRACTOR_NAME: &str = "image-link-extractor";

    fn strategy_a(&self, resource_str: &str) -> Vec<String> {
        return strategy_a_common_extractor(
            resource_str,
            self.extensions.clone(),
            &self.domain,
            self.is_same_domain_enabled,
            self.processing_page_link.to_owned(),
        );
    }

    fn strategy_b(&self, resource_str: &str) -> Vec<String> {
        return Document::from(resource_str)
            .find(Name("img"))
            .filter_map(|n| n.attr("src"))
            .map(|item| item.to_string())
            .map(|link| {
                add_base_url_if_not_present(&link, &self.domain, &self.processing_page_link)
            })
            .filter_map(|link| {
                is_same_domain_ext(self.is_same_domain_enabled, self.domain.as_str(), &link)
            })
            .filter_map(|link| contains_extension(self.extensions.clone(), &link))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_b_tag_and_attribute_upper_case() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".png".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                    <IMG SRC="ciao.png" />
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!(
            "http://dodu.it/test/ciao.png",
            result.iter().next().unwrap()
        );
    }

    #[test]
    fn strategy_b_test_tag_and_attribute_lower_case() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".png".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                    <img src="ciao.png" />
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!(
            "http://dodu.it/test/ciao.png",
            result.iter().next().unwrap()
        );
    }

    #[test]
    fn strategy_b_test_domain_root() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".png".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/index.html".to_owned(),
        };
        let resource_str = r#"
                    <IMG SRC="/ciao.png" />
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!("http://dodu.it/ciao.png", result.iter().next().unwrap());
    }

    #[test]
    fn strategy_b_link_root_test() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".jpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                <h1>/ciao.jpg</h1>
                <figure class="wp-block-image size-large">
                    <img src="/ciao.jpg" alt=""/>
                </figure>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!("http://dodu.it/ciao.jpg", result.iter().next().unwrap());
    }

    #[test]
    fn strategy_b_link_relative_test() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".jpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                <h1>/ciao.jpg</h1>
                <figure class="wp-block-image size-large">
                    <img src="ciao.jpg" alt=""/>
                </figure>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!(
            "http://dodu.it/test/ciao.jpg",
            result.iter().next().unwrap()
        );
    }

    #[test]
    fn strategy_b_link_relative_second_test() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".jpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                <h1>/ciao.jpg</h1>
                <figure class="wp-block-image size-large">
                    <img src="./ciao.jpg" alt=""/>
                </figure>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!(
            "http://dodu.it/test/ciao.jpg",
            result.iter().next().unwrap()
        );
    }

    #[test]
    fn strategy_b_link_absolute_double_slash_test() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".jpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                <h1>/ciao.jpg</h1>
                <figure class="wp-block-image size-large">
                    <img src="//dodu.it/ciao.jpg" alt=""/>
                </figure>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!("http://dodu.it/ciao.jpg", result.iter().next().unwrap());
    }

    #[test]
    fn strategy_b_link_img_and_link_test() {
        let audio_extractor = ImageLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".jpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_owned(),
        };
        let resource_str = r#"
                <h1>/ciao.jpg</h1>
                <figure class="wp-block-image size-large">
                    <img src="ciao.jpg" alt=""/>
                </figure>
                <a href="ciao-ciao.jpg" target="_blank">Download</a>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(2, result.len());
        assert_eq!(true, result.get("http://dodu.it/test/ciao.jpg").is_some());
        assert_eq!(
            true,
            result.get("http://dodu.it/test/ciao-ciao.jpg").is_some()
        );
    }
}
