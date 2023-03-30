use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::{fs, io};

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
        let mut file = fs::File::open(&path).unwrap();
        let mut hasher = Sha256::new();
        let n = io::copy(&mut file, &mut hasher).unwrap();
        let hash = hasher.finalize();
        let result = hex::encode(hash);
        return result;
    }
}
