use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub struct Config {
    pub prompt: String,
}

// Read and deserialize configuration
pub fn read_config(config_path: &str) -> Config {
    let file = File::open(config_path).unwrap();
    let reader = BufReader::new(file);
    let config_file: Value = serde_json::from_reader(reader).unwrap();
    let config_shell = Config {
            prompt: config_file["prompt"].as_str().unwrap().to_string() };
    config_shell
}
