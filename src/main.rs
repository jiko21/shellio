mod util;
mod spec;

use clap::{App, load_yaml};
use std::fs;
use similar::{ChangeTag, TextDiff};
use crate::spec::spec::SpecFile;
use std::fs::File;
use std::io::{BufWriter, Write};

fn save_spec(path: &str, spec: SpecFile) -> Result<(), String> {
    let spec_file = format!("{}\n", serde_json::to_string_pretty(&spec).unwrap());
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(_) => {
            return Err("Cannot save file".to_string());
        }
    };
    let mut writer = BufWriter::new(file);
    match writer.write(&spec_file.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Cannot save file".to_string())
    }
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
    let mut spec_file = SpecFile::new(config.as_str());
    let summary = spec_file.execute();
    match save_spec(spec_path, spec_file) {
        Ok(_) => {},
        Err(e) => {
            panic!(e);
        }
    };
    println!("Total: {}\t Success: {}\t Fail: {}\tNew: {}", summary.total, summary.success, summary.fail, summary.new);
}
