// This file contains the code that search fetch and filter desktop entries (some only want to show in certain
// conditions)
//
// The fetching is very simple and only use a small portion of all informations contained in the
// .desktop files

use std::{env, fs, str};

// Fetch .desktop entries in common directories
#[allow(deprecated)] // Only for linux for now
#[cfg(target_os = "linux")]
pub(crate) fn fetch_entries_to_string() -> Vec<String> {
    let directories: Vec<String> = vec![
        // format!("{}", env::home_dir().unwrap_or_default().to_str().unwrap()),
        String::from("/usr/share/applications/"),
        String::from("/usr/local/share/applications/"),
    ];

    let mut entries: Vec<String> = vec![];

    // Add every entries to a vector to fetch them after, not the most efficient but seems clear
    for dir in directories.iter() {
        if let Ok(read_dir) = fs::read_dir(dir) {
            for entry in read_dir {
                let path = entry.unwrap().path();
                if let Some(path_str) = path.to_str() {
                    if !path_str.contains(".desktop") {
                        continue;
                    }
                    entries.push(path_str.to_string());
                }
            }
        }
    }

    entries
}

#[derive(Default, Debug, PartialEq, Clone)]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) img_path: String,
    pub(crate) entry_path: String,
    pub(crate) key_words: Vec<String>,
    pub(crate) only_show_in: Option<String>,
    // TODO: Ajouter NotShowIn qui fait l'opposé de OnlyShowIn
    pub(crate) r#type: String, // type is a reserved keyword, r# allows to use it apparently
}

impl Entry {
    // Parse Key words from the .desktop file, they are used as aliases to search for the apps
    pub(crate) fn parse_keywords(&mut self, input: &str) {
        // Not really pretty to look at
        // Basically concats with [self.key_words, new_key_words].concat()
        self.key_words.extend(
            input
                .split([',', ' ', ';']) // Different separators that appear in the files i've seen
                // filter map to filter out empty strings that appear
                .filter(|word| !word.is_empty())
                .map(|word| word.to_string()),
        )
    }
}

// Takes a vector of paths (as Strings) and fetch them 1 by 1, returning a vector of Entries
pub(crate) fn fetch_entries_from_paths(paths: Vec<String>) -> Vec<Entry> {
    // Get current desktop environment to filter out unneeded entries
    let desktop_env = env::var("XDG_CURRENT_DESKTOP");

    let mut out: Vec<Entry> = vec![];
    for path in paths.iter() {
        let entry = fetch_entry_from_path(path.to_string());

        // If the entry is none, go to next iteration without adding the entry
        if entry.is_none() {
            continue;
        }
        let entry = entry.unwrap();

        // Filters out unneeded entries relative to desktop environment
        // If condition == true -> jump to next iteration without pushing to out vector
        if let (Some(only), Ok(env)) = (&entry.only_show_in, &desktop_env) {
            // Handle multiple values in OnlyShowIn, de is for Desktop Environment
            if !only
                .split(';')
                .any(|de| de.trim().to_lowercase() == env.to_lowercase())
            {
                continue;
            }
        }

        out.push(entry);
    }
    out
}

// Read entry file and gives the data to fetch_from_entry_bytes function
pub(crate) fn fetch_entry_from_path(path: String) -> Option<Entry> {
    let buf = fs::read(path.clone());
    if buf.is_err() {
        return None;
    }
    match fetch_entry_from_bytes(buf.unwrap()) {
        Some(mut out) => {
            out.entry_path = path; // Adds an entry's path before returning it
            Some(out)
        }
        None => None, // If none, then none... Mind blown
    }
}

// Actual fetching of the entries
pub(crate) fn fetch_entry_from_bytes(input: Vec<u8>) -> Option<Entry> {
    let mut output = Entry::default();
    // We consider the file valid UTF8 because i'm lazy
    for line in String::from_utf8_lossy(&input).lines() {
        // End fetching if file contains another section than [Desktop Entry] (because i'm lazy)
        if line.contains("[Desktop Action") {
            break;
        }

        // Ignore commented lines
        if line.starts_with("#") {
            continue;
        }

        // Split lines on '=' and parse {key,value} pairs
        let mut split = line.split("=");
        if let Some(val1) = split.next()
            && let Some(val2) = split.next()
        {
            // Matches keys and assign the values on the Entry outputed
            match val1 {
                "NoDisplay" => {
                    // If NoDisplay is true then return None to ignore the entry
                    if val2.to_lowercase() == "true" {
                        return None;
                    }
                }
                "OnlyShowIn" => output.only_show_in = Some(val2.to_string()),
                "Name" => output.name = val2.to_string(),
                "Type" => output.r#type = val2.to_string(),
                "Icon" => output.img_path = val2.to_string(),
                "Comment" | "Categories" | "Description" | "Keywords" => {
                    output.parse_keywords(val2)
                }
                _ => (),
            }
        }
    }

    Some(output)
}
