use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn write_to_terminal_multicolor(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let buffer_writer = BufferWriter::stderr(ColorChoice::Always);
    // let mut buffer = buffer_writer.buffer();
    // writeln!(&mut buffer, "{}", text).expect("an error occurred in terminal writer");
    // buffer_writer.print(&buffer).expect("an error occurred in terminal writer");
    // Ok(())
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "{}", text).expect("TODO: panic message");
    Ok(())
}

// macro_rules! hash {
//     ($($key:expr => $value:expr), *) => {
//         let mut hashmap = ::std::collections::HashMap::new();
//         $(hashmap.insert($key, $value);)*
//         hashmap
//     }
// }
