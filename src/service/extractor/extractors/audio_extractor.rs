use select::{
    document::Document,
    predicate::{self, Name, Predicate},
};

use crate::util::{
    link_util::{add_base_url_if_not_present, normalize_link_replace_spaces},
    validation_util::is_same_domain_ext,
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

impl AudioExtractor {
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
        let predicate = predicate::Name("audio").child(predicate::Name("source"));
        let audio2: Vec<String> = Document::from(resource_str)
            .find(predicate)
            .filter_map(|n| n.attr("src"))
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
        audio2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_b_test() {
        let audio_extractor = AudioExtractor {
            domain: "http://dodu.it".to_owned(),
            enabled: true,
            extensions: vec![".ogg".to_owned(), ".mp3".to_owned(), ".mid".to_owned()],
            is_same_domain_enabled: false,
        };
        let resource_str = r#"
                    <audio controls>
                        <source src="horse.ogg" type="audio/ogg">
                        <source src="horse.mp3" type="audio/mpeg">
                        Your browser does not support the audio element.
                    </audio>
        "#;
        let result = audio_extractor.extract(resource_str);
        assert_eq!(2, result.len());
        assert_eq!("http://dodu.it/horse.ogg", result.get(0).unwrap());
        assert_eq!("http://dodu.it/horse.mp3", result.get(1).unwrap());
    }
}
