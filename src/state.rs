use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};



pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("unable to write file");
}
