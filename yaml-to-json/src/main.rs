use std::{
    error::Error,
    fs::File,
    io::{self, stdin, stdout, Read, Write},
};

use clap::{builder::EnumValueParser, Parser};
use color::ColorOpt;
use colorize_json::ColorizedSerializerBuilder;
use serde::Serialize;
use serde_json::{
    ser::{CompactFormatter, PrettyFormatter},
    to_value,
};
use termcolor::{Buffer, BufferWriter, ColorChoice};

mod color;

/// Convert yaml to json
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Minify json output
    #[clap(short, long, value_parser)]
    minify: bool,

    /// Colorize json output
    #[clap(
        short,
        long,
        value_parser = EnumValueParser::<ColorOpt>::new(),
        default_value_t = ColorOpt::Auto,
    )]
    color: ColorOpt,

    /// Path to a yaml file (default: stdin)
    #[clap(value_parser)]
    file: Option<String>,
}

trait JsonPrinter {
    fn print(&self) -> io::Result<()>;
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
    let color_choice = ColorChoice::from(args.color);
    if color_choice == ColorChoice::Never {
        struct StringJsonPrinter(String);
        impl JsonPrinter for StringJsonPrinter {
            fn print(&self) -> io::Result<()> {
                stdout().write_all(self.0.as_bytes())
            }
        }

        if args.minify {
            print_json(serde_json::to_string(&yaml).map(StringJsonPrinter));
        } else {
            print_json(serde_json::to_string_pretty(&yaml).map(StringJsonPrinter));
        }
    } else {
        struct TermcolorJsonPrinter(BufferWriter, Buffer);
        impl JsonPrinter for TermcolorJsonPrinter {
            fn print(&self) -> io::Result<()> {
                self.0.print(&self.1)
            }
        }

        print_json(to_value(&yaml).and_then(|val| {
            let out = BufferWriter::stdout(color_choice);
            let mut buf = out.buffer();
            if args.minify {
                let mut ser =
                    ColorizedSerializerBuilder::with_formatter(&mut buf, CompactFormatter).build();
                val.serialize(&mut ser)?;
            } else {
                let mut ser = ColorizedSerializerBuilder::with_formatter(
                    &mut buf,
                    PrettyFormatter::default(),
                )
                .build();
                val.serialize(&mut ser)?;
            }
            Ok(TermcolorJsonPrinter(out, buf))
        }));
    }
}

fn print_json<P: JsonPrinter, E: Error>(json: Result<P, E>) {
    json.expect("Failed to convert to json")
        .print()
        .expect("Failed to print json");
}
