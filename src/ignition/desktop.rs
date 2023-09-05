use crate::util::write_to_terminal_multicolor;
use crate::frontend::desktop::app::start_desktop;


pub fn ignite_desktop() {
    write_to_terminal_multicolor("Starting up desktop application ...").expect("TODO: panic message");

    start_desktop()
}

