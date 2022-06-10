use std::io::{Result, Write};

use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

pub fn highlight(syntax: &str, source: &str, writer: &mut impl Write) -> Result<()> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(syntax).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);
    for line in LinesWithEndings::from(source) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        writer.write_all(escaped.as_bytes())?;
    }

    Ok(())
}
