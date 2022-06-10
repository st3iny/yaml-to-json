use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

use atty::Stream;
use clap::Parser;
use color::highlight;

/// Convert yaml to json
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Minify json output
    #[clap(short, long)]
    minify: bool,

    /// Highlight json output
    #[clap(short, long, conflicts_with = "no-color")]
    color: bool,

    /// Don't highlight json output
    #[clap(short, long, conflicts_with = "color")]
    no_color: bool,

    /// Path to a yaml file (default: stdin)
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

    let yaml: serde_yaml::Value = serde_yaml::from_reader(file).expect("Failed to parse yaml");
    let json = match args.minify {
        true => serde_json::to_string(&yaml),
        false => serde_json::to_string_pretty(&yaml),
    }
    .expect("Failed to convert to json");

    if args.color || (!args.no_color && atty::is(Stream::Stdout)) {
        highlight("json", &json, &mut stdout()).expect("Failed to highlight and write json");
    } else {
        stdout()
            .write_all(json.as_bytes())
            .expect("Failed to write json");
    }
}
