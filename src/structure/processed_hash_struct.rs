use std::collections::HashSet;

use crate::util::file_util::calculate_file_hash;

pub struct ProcessedHash {
    processed_resources: HashSet<String>,
    enabled: bool,
}

impl ProcessedHash {
    pub fn new(enabled: bool) -> ProcessedHash {
        ProcessedHash {
            processed_resources: HashSet::new(),
            enabled,
        }
    }

    pub fn was_already_processed(&self, path: &str) -> bool {
        if !self.enabled {
            return false;
        }
        self.processed_resources
            .contains(&ProcessedHash::calculate_hash(path))
    }

    pub fn push(&mut self, path: &str) {
        if !self.enabled {
            return;
        }
        self.processed_resources
            .insert(ProcessedHash::calculate_hash(path));
    }

    fn calculate_hash(path: &str) -> String {
        calculate_file_hash(path)
    }
}
