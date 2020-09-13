use std::io::{self, Write};
use termcolor::{BufferWriter, ColorChoice, WriteColor, Color, ColorSpec};

pub fn write_to_terminal_multicolor(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer_writer = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = buffer_writer.buffer();
    writeln!(&mut buffer, "{}", text)?;
    buffer_writer.print(&buffer)?;
    Ok(())
}