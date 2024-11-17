use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

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

pub fn envkeys(path: &str, key: &str) -> Result<Option<String>, Error> {
    let env_variables = read_file(path)?;

    let api_key = env_variables.get(key).map(|key_value| key_value.clone());

    if api_key.is_none() {
        println!("Key is not defined.");
    }

    Ok(api_key)
}
