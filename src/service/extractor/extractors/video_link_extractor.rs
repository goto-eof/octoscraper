use super::resource_extractor::{strategy_a_common_extractor, ResourceExtractor};
use crate::util::{
    link_util::{add_base_url_if_not_present, has_extension, normalize_link_replace_spaces},
    validation_util::is_same_domain_ext,
};
use select::{
    document::Document,
    predicate::{self, Predicate},
};
use std::collections::HashSet;

pub struct VideoLinkExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
    pub processing_page_link: String,
}

impl ResourceExtractor for VideoLinkExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_name(&self) -> String {
        return VideoLinkExtractor::EXTRACTOR_NAME.to_string();
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

impl VideoLinkExtractor {
    pub const EXTRACTOR_NAME: &str = "video-link-extractor";

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
        let predicate = predicate::Name("video").child(predicate::Name("source"));
        return Document::from(resource_str)
            .find(predicate)
            .filter_map(|n| n.attr("src"))
            .map(|item| item.to_string())
            .filter_map(|link| has_extension(&link, self.extensions.clone()))
            .map(|link| {
                add_base_url_if_not_present(&link, &self.domain, &self.processing_page_link)
            })
            .filter_map(|link| normalize_link_replace_spaces(&link))
            .filter_map(|link| is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link))
            .collect();
    }
}

// =========================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_b_test_extract_from_video_tag() {
        let audio_extractor = VideoLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".mp4".to_owned(), ".ogg".to_owned(), "mpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_string(),
        };
        let resource_str = r#"
                <video width="320" height="240" controls>
                    <source src="movie.mp4" type="video/mp4">
                    Your browser does not support the video tag.
                </video>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(1, result.len());
        assert_eq!(
            "http://dodu.it/test/movie.mp4",
            result.iter().next().unwrap()
        );
    }

    #[test]
    fn strategy_b_test_extract_from_video_tag_2_links() {
        let audio_extractor = VideoLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".mp4".to_owned(), ".ogg".to_owned(), "mpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_string(),
        };
        let resource_str = r#"
                <video width="320" height="240" controls>
                    <source src="movie.mp4" type="video/mp4">
                    <source src="movie.ogg" type="video/ogg">
                    Your browser does not support the video tag.
                </video>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(2, result.len());
        assert_eq!(true, result.get("http://dodu.it/test/movie.mp4").is_some());
        assert_eq!(true, result.get("http://dodu.it/test/movie.ogg").is_some());
    }

    #[test]
    fn strategy_b_test_find_by_extension() {
        let audio_extractor = VideoLinkExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".mp4".to_owned(), ".ogg".to_owned(), "mpg".to_owned()],
            is_same_domain_enabled: false,
            processing_page_link: "http://dodu.it/test/index.html".to_string(),
        };
        let resource_str = r#"
                <video width="320" height="240" controls>
                    <source src="movie.uknown" type="video/mp4">
                    Your browser does not support the video tag.
                </video>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(0, result.len());
    }
}
