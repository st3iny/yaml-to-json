use std::fmt::Display;

use clap::{PossibleValue, ValueEnum};
use termcolor::ColorChoice;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorOpt {
    Auto,
    Always,
    Never,
}

impl ValueEnum for ColorOpt {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Auto, Self::Always, Self::Never]
    }

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        match self {
            Self::Auto => Some(PossibleValue::new("auto")),
            Self::Always => Some(PossibleValue::new("always")),
            Self::Never => Some(PossibleValue::new("never")),
        }
    }
}

impl Display for ColorOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        };
        f.write_str(name)
    }
}

impl From<ColorOpt> for ColorChoice {
    fn from(val: ColorOpt) -> Self {
        match val {
            ColorOpt::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                }
            }
            ColorOpt::Always => ColorChoice::Always,
            ColorOpt::Never => ColorChoice::Never,
        }
    }
}
