extern crate serde;
extern crate serde_json;

use clap::{App, load_yaml};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Spec {
    describe: String,
    command: String,
    results: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpecFile {
    name: String,
    specs: Vec<Spec>,
}

fn main() {
    let yaml = load_yaml!("./cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let spec_path = match matches.value_of("SPECFILE") {
        Some(path) => path,
        None => "shellio.spec.json",
    };
    let mut config = match fs::read_to_string(spec_path) {
        Ok(file) => file,
        Err(_) => {
            panic!("Cannot read file: {}. You should add shellio.spec.json or specify shellio spec file path.", spec_path);
        }
    };
    let spec_file: SpecFile = match serde_json::from_str(config.as_str()) {
        Ok(str) => str,
        Err(e) => {
            panic!("Error while parsing specfile: {}", e)
        }
    };
    println!("{:?}", spec_file);
}
