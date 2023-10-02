use serde_json::{json, value::Value, Map};
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    println!("{:?}", json);
    let state = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    println!("{:?}", new_data);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}