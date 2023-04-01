use select::{
    document::Document,
    predicate::{self, Name, Predicate},
};

use crate::util::{
    link_util::{add_base_url_if_not_present, has_extension, normalize_link_replace_spaces},
    validation_util::is_same_domain_ext,
};

use super::resource_extractor::ResourceExtractor;

pub struct VideoExtractor {
    pub enabled: bool,
    pub extensions: Vec<String>,
    pub is_same_domain_enabled: bool,
    pub domain: String,
}

// TODO
impl ResourceExtractor for VideoExtractor {
    fn enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn extract(&self, resource_str: &str) -> Vec<String> {
        let mut links: Vec<String> = Vec::new();

        if self.enabled {
            self.strategy_a(resource_str)
                .iter()
                .for_each(|elem| links.push(elem.to_string()));
            self.strategy_b(resource_str)
                .iter()
                .for_each(|elem| links.push(elem.to_string()));
        }

        return links;
    }
}

impl VideoExtractor {
    fn strategy_a(&self, resource_str: &str) -> Vec<String> {
        return Document::from(resource_str)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|item| item.to_string())
            .filter(|link| {
                for extension in self.extensions.iter() {
                    if link.ends_with(extension) {
                        return true;
                    }
                }
                return false;
            })
            .map(|link| add_base_url_if_not_present(&link, &self.domain))
            .filter_map(|link| normalize_link_replace_spaces(&link))
            .filter_map(|link| is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link))
            .collect();
    }

    fn strategy_b(&self, resource_str: &str) -> Vec<String> {
        let predicate = predicate::Name("video").child(predicate::Name("source"));
        return Document::from(resource_str)
            .find(predicate)
            .filter_map(|n| n.attr("src"))
            .map(|item| item.to_string())
            .filter_map(|link| has_extension(&link, self.extensions.clone()))
            .map(|link| add_base_url_if_not_present(&link, &self.domain))
            .filter_map(|link| normalize_link_replace_spaces(&link))
            .filter_map(|link| is_same_domain_ext(self.is_same_domain_enabled, &self.domain, &link))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_b_test() {
        let audio_extractor = VideoExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".mp4".to_owned(), ".ogg".to_owned(), "mpg".to_owned()],
            is_same_domain_enabled: false,
        };
        let resource_str = r#"
                <video width="320" height="240" controls>
                    <source src="movie.mp4" type="video/mp4">
                    <source src="movie.ogg" type="video/ogg">
                    Your browser does not support the video tag.
                </video>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!("http://dodu.it/movie.mp4", result.get(0).unwrap());
        assert_eq!(2, result.len());
        assert_eq!("http://dodu.it/movie.ogg", result.get(1).unwrap());
    }
}
