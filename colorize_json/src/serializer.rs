use std::io::{self, Write};

use serde_json::{
    ser::{CompactFormatter, Formatter, PrettyFormatter},
    Serializer,
};
use termcolor::{Color, ColorSpec, WriteColor};

pub struct NoopWriter;

impl Write for NoopWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct ColorizedSerializerBuilder<'w, W, F>
where
    W: WriteColor,
    F: Formatter,
{
    obj_key_color: ColorSpec,
    string_color: ColorSpec,
    null_color: ColorSpec,
    formatter: F,
    writer: &'w mut W,
}

impl<'w, W> ColorizedSerializerBuilder<'w, W, CompactFormatter>
where
    W: WriteColor,
{
    pub fn new(writer: &'w mut W) -> Self {
        Self::with_formatter(writer, CompactFormatter)
    }
}

impl<'w, W> ColorizedSerializerBuilder<'w, W, PrettyFormatter<'_>>
where
    W: WriteColor,
{
    pub fn pretty(writer: &'w mut W) -> Self {
        Self::with_formatter(writer, PrettyFormatter::default())
    }
}

impl<'w, W, F> ColorizedSerializerBuilder<'w, W, F>
where
    W: WriteColor,
    F: Formatter,
{
    pub fn with_formatter(writer: &'w mut W, formatter: F) -> Self {
        let mut obj_key_color = ColorSpec::new();
        obj_key_color.set_fg(Some(Color::Blue));
        obj_key_color.set_bold(true);
        let mut string_color = ColorSpec::new();
        string_color.set_fg(Some(Color::Green));
        string_color.set_bold(true);
        let mut null_color = ColorSpec::new();
        null_color.set_fg(Some(Color::Black));
        null_color.set_bold(true);
        Self {
            obj_key_color,
            string_color,
            null_color,
            formatter,
            writer,
        }
    }

    pub fn with_object_key_color(self, obj_key_color: ColorSpec) -> Self {
        Self {
            obj_key_color,
            ..self
        }
    }

    pub fn with_string_color(self, string_color: ColorSpec) -> Self {
        Self {
            string_color,
            ..self
        }
    }

    pub fn with_null_color(self, null_color: ColorSpec) -> Self {
        Self { null_color, ..self }
    }

    pub fn build(self) -> Serializer<NoopWriter, ColorizedFormatter<'w, W, F>> {
        let formatter = ColorizedFormatter {
            obj_key_color: self.obj_key_color,
            string_color: self.string_color,
            null_color: self.null_color,
            formatter: self.formatter,
            writer: self.writer,
            is_object_key: false,
        };
        Serializer::with_formatter(NoopWriter, formatter)
    }
}

pub struct ColorizedFormatter<'w, W, F>
where
    W: WriteColor,
    F: Formatter,
{
    obj_key_color: ColorSpec,
    string_color: ColorSpec,
    null_color: ColorSpec,
    writer: &'w mut W,
    formatter: F,
    is_object_key: bool,
}

impl<'w, WC, F> Formatter for ColorizedFormatter<'w, WC, F>
where
    WC: WriteColor,
    F: Formatter,
{
    fn write_null<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.writer.set_color(&self.null_color)?;
        self.formatter.write_null(&mut self.writer)?;
        self.writer.reset()?;
        Ok(())
    }

    fn begin_string<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.is_object_key {
            self.writer.set_color(&self.obj_key_color)?;
        } else {
            self.writer.set_color(&self.string_color)?;
        }
        self.formatter.begin_string(&mut self.writer)?;
        Ok(())
    }

    fn end_string<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_string(&mut self.writer)?;
        self.writer.reset()?;
        Ok(())
    }

    fn begin_object_key<W>(&mut self, _writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.is_object_key = true;
        self.formatter.begin_object_key(&mut self.writer, first)?;
        Ok(())
    }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.is_object_key = false;
        self.formatter.end_object_key(&mut self.writer)?;
        Ok(())
    }

    fn write_bool<W>(&mut self, _writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_bool(&mut self.writer, value)
    }

    fn write_i8<W>(&mut self, _writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_i8(&mut self.writer, value)
    }

    fn write_i16<W>(&mut self, _writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_i16(&mut self.writer, value)
    }

    fn write_i32<W>(&mut self, _writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_i32(&mut self.writer, value)
    }

    fn write_i64<W>(&mut self, _writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_i64(&mut self.writer, value)
    }

    fn write_u8<W>(&mut self, _writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_u8(&mut self.writer, value)
    }

    fn write_u16<W>(&mut self, _writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_u16(&mut self.writer, value)
    }

    fn write_u32<W>(&mut self, _writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_u32(&mut self.writer, value)
    }

    fn write_u64<W>(&mut self, _writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_u64(&mut self.writer, value)
    }

    fn write_f32<W>(&mut self, _writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_f32(&mut self.writer, value)
    }

    fn write_f64<W>(&mut self, _writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_f64(&mut self.writer, value)
    }

    fn write_number_str<W>(&mut self, _writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.write_number_str(&mut self.writer, value)
    }

    fn write_string_fragment<W>(&mut self, _writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter
            .write_string_fragment(&mut self.writer, fragment)
    }

    fn write_char_escape<W>(
        &mut self,
        _writer: &mut W,
        char_escape: serde_json::ser::CharEscape,
    ) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter
            .write_char_escape(&mut self.writer, char_escape)
    }

    fn begin_array<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.begin_array(&mut self.writer)
    }

    fn end_array<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_array(&mut self.writer)
    }

    fn begin_array_value<W>(&mut self, _writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.begin_array_value(&mut self.writer, first)
    }

    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_array_value(&mut self.writer)
    }

    fn begin_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.begin_object(&mut self.writer)
    }

    fn end_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_object(&mut self.writer)
    }

    fn begin_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.begin_object_value(&mut self.writer)
    }

    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_object_value(&mut self.writer)
    }

    fn write_raw_fragment<W>(&mut self, _writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter
            .write_raw_fragment(&mut self.writer, fragment)
    }
}
