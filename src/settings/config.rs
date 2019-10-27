use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Config {
    pub prompt: String,
}

// Read and deserialize configuration
pub fn read_config(config_path: &str) -> Config {
    let config  =
        if Path::new(config_path).exists() {
            let file = File::open(config_path).unwrap();
            let reader = BufReader::new(file);
            let config_file  = serde_json::from_reader(reader);
            if config_file.is_ok() {
                let config: Value = config_file.unwrap();
                Config { prompt: config["prompt"].as_str()
                                         .unwrap()
                                         .to_string() }
            }
            else {
                eprintln!("Configuration Error: {}: {:?}",
                          config_path,
                          config_file.err().unwrap().to_string());
                Config { prompt: "$ ".to_string() }
            }
        } else {
            Config { prompt: "$ ".to_string() }
        };
    config
}
