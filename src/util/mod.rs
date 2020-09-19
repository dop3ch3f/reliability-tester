use std::io::{self, Write};
use termcolor::{BufferWriter, ColorChoice};

pub fn write_to_terminal_multicolor(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let buffer_writer = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = buffer_writer.buffer();
    writeln!(&mut buffer, "{}", text).expect("an error occurred in terminal writer");
    buffer_writer.print(&buffer).expect("an error occurred in terminal writer");
    Ok(())
}