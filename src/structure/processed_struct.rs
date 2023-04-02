use reqwest::Url;
use std::collections::HashSet;

pub struct Processed {
    processed_resources: HashSet<String>,
    unique_process_method: UniqueProcessMethod,
}

pub enum UniqueProcessMethod {
    FileUnique = 1,
    LinkUnique = 2,
}

impl Processed {
    pub fn new(unique_process_method: UniqueProcessMethod) -> Processed {
        Processed {
            processed_resources: HashSet::new(),
            unique_process_method,
        }
    }

    pub fn was_already_processed(&self, link: &str) -> bool {
        self.processed_resources.contains(&self.retrieve_name(link))
    }

    fn retrieve_name(&self, link: &str) -> String {
        match self.unique_process_method {
            UniqueProcessMethod::FileUnique => Processed::extract_fname(link, None),
            UniqueProcessMethod::LinkUnique => link.to_owned(),
        }
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
