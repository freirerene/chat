use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Deserialize)]
struct Preferences {
    llm: String,
    model: String,
}

pub fn read_preferences(file_path: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let preferences: Preferences = serde_json::from_str(&file_content)?;
    Ok((preferences.llm, preferences.model))
}

pub fn envkeys(path: &str, key: &str) -> Result<Option<String>, io::Error> {
    let env_variables = read_file(path)?;

    let api_key = env_variables.get(key).map(|key_value| key_value.clone());

    if api_key.is_none() {
        println!("Key is not defined.");
    }

    Ok(api_key)
}

fn read_file(filename: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(map)
}
