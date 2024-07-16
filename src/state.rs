use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create(file_name).expect("Unable to create file");
            new_file.write_all(b"{}").expect("Unable to write to file");
            new_file
        }
    };

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");

    if data.trim().is_empty() {
        data = "{}".to_string();
    }

    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let state = json.as_object().expect("Expected JSON object").clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("unable to write file");
}
