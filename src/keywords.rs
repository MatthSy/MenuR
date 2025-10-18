use crate::entries::Entry;
use std::collections::{hash_map, HashMap};

#[derive(Debug, Clone)]
pub(crate) struct Keywords(hash_map::HashMap<String, Vec<String>>);

impl Keywords {
    pub(crate) fn new() -> Self {
        Keywords(HashMap::default())
    }

    pub(crate) fn _from(entries: &[Entry]) -> Self {
        let mut result = Self::new();

        for entry in entries {
            result.gen_keywords_for_entry(entry);
        }
        result
    }

    pub(crate) fn gen_keywords_for_entry(&mut self, entry: &Entry) {
        let name = entry.name.clone().to_lowercase();

        // First : add entries' names as a keyword :
        if self.0.contains_key(&name) {
            // If keyword already exists
            self.0.get_mut(&name).unwrap().push(name.clone());
        } else {
            self.0.insert(name.clone(), vec![name.clone()]);
        }

        // Then add actual keywords :
        for keyword in &entry.key_words {
            let keyword = &keyword.clone().to_lowercase();
            if self.0.contains_key(keyword.as_str()) {
                // If keyword already exists
                self.0.get_mut(keyword).unwrap().push(name.clone());
            } else {
                self.0.insert(keyword.to_string(), vec![name.clone()]);
            }
        }
    }

    pub(crate) fn match_keywords(&self, search: &str) -> Vec<&str> {
        let mut matching: Vec<&str> = vec![];

        for word in search.to_lowercase().trim().split(" ") {
            if word.is_empty() {
                continue;
            }
            for key in self.0.keys() {
                if key.starts_with(word) {
                    let entries = self.0.get(key).unwrap();
                    for entry in entries {
                        println!("{}", &entry);
                        matching.push(entry);
                    }
                }
            }
        }

        matching
    }
}
