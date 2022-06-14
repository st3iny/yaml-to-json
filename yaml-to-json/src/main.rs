use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

use clap::Parser;

/// Convert yaml to json
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Minify json output
    #[clap(short, long, value_parser)]
    minify: bool,

    /// Path to a yaml file (default: stdin)
    #[clap(value_parser)]
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

    let yaml: serde_yaml::Value = serde_yaml::from_reader(file).expect("Failed to parse yaml");
    let json = match args.minify {
        true => serde_json::to_string(&yaml),
        false => serde_json::to_string_pretty(&yaml),
    }
    .expect("Failed to convert to json");

    stdout()
        .write_all(json.as_bytes())
        .expect("Failed to write json");
}
