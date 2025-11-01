use std::{env, io};
use crate::entries::Entry;

fn get_cache_dir_file() -> (String, String) {
    let dir_path = format!(
        "{}/.cache/menur/",
        env::home_dir().unwrap_or_default().to_str().unwrap()
    );
    let file_path = format!("{}entries.cache", dir_path);

    (dir_path, file_path)
}

pub(crate) fn put_to_cache(entries: Vec<Entry>) -> io::Result<()> {
    let (dir_path, file_path) = get_cache_dir_file();
    std::fs::create_dir_all(dir_path)?;

    let serialized_data = create_cache_data(entries);
    std::fs::write(file_path, serialized_data.as_bytes())
}

pub(crate) fn create_cache_data(entries: Vec<Entry>) -> String {
    let mut result = String::new();

    for entry in entries {
        let serialized_entry = format!(
            "name: {}; img_path: {}; entry_path: {}; key_words: {:?}; only_show_in: {:?}; type: {}",
            entry.name,
            entry.img_path,
            entry.entry_path,
            entry.key_words,
            entry.only_show_in,
            entry.r#type
        );
        result = format!("{result}{serialized_entry}\n");
    }

    result
}

pub(crate) fn read_from_cache() -> Option<Vec<Entry>> {
    let (_dir_path, file_path) = get_cache_dir_file();
    if let Ok(val) = std::fs::exists(&file_path) && val {} else {
        return None;
    };

    let mut res: Vec<Entry> = vec![];
    for line in std::fs::read_to_string(&file_path).expect("Cache File could not be read.").lines() {
        res.push(parse_entry_cache_line(line));
    }
    Some(res)
}

fn parse_entry_cache_line(line: &str) -> Entry {
    let mut res = Entry::default();
    for pair in line.split(';') {
        let mut pair = pair.split(": ");
        let key = pair.next().unwrap_or("").trim();
        let value = pair.next().unwrap_or("").trim();

        match key {
            "name" => res.name = value.to_string(),
            "img_path" => res.img_path = value.to_string(),
            "entry_path" => res.entry_path = value.to_string(),
            "key_words" => {
                let value = value.trim_matches(|c| c == '[' || c == ']');
                for keyword in value.split(", ") {
                    res.key_words.push(keyword.trim_matches('"').to_string());
                }
            },
            // Only show in is not needed here because entries that should not be shown are not
            // cached to begin with
            "type" => res.r#type = value.to_string(),

            _ => {}
        }
    }


    res
}
