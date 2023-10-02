use std::env;

use process::process_input;
use serde_json::{json, Map, Value};
use state::{read_file, write_to_file};
use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{create::Create, delete::Delete, get::Get},
    ItemTypes,
};
mod process;
mod state;
mod to_do;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_owned();
        }
    }
    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state);
}
