use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

use clap::Parser;

/// Convert json to yaml
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Path to a json file (default: stdin)
    #[clap()]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file: Box<dyn Read> = match args.file.as_deref() {
        Some("-") | None => Box::new(stdin()),
        Some(file) => {
            Box::new(File::open(file).unwrap_or_else(|_| panic!("Failed to open {}", file)))
        }
    };

    let json: serde_json::Value = serde_json::from_reader(file).expect("Failed to parse json");
    let yaml = serde_yaml::to_string(&json).expect("Failed to convert to yaml");

    stdout()
        .write_all(yaml.as_bytes())
        .expect("Failed to write yaml");
}
