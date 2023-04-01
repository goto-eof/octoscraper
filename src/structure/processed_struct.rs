use reqwest::Url;
use std::collections::HashSet;

pub struct Processed {
    processed_resources: HashSet<String>,
}

impl Processed {
    pub fn new() -> Processed {
        Processed {
            processed_resources: HashSet::new(),
        }
    }

    pub fn was_already_processed(&self, link: &str) -> bool {
        self.processed_resources
            .contains(&Processed::extract_fname(link, None))
    }

    pub fn push(&mut self, link: &str) {
        self.processed_resources
            .insert(Processed::extract_fname(link, None));
    }

    pub fn extract_fname(link: &str, alternative_file_name: Option<String>) -> String {
        let alternative_name = if alternative_file_name.is_some() {
            alternative_file_name.unwrap()
        } else {
            "no_filename".to_string()
        };
        let fname = Url::parse(link)
            .unwrap()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or(&alternative_name)
            .to_string();
        return fname;
    }
}
