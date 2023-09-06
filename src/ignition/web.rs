use crate::util::write_to_terminal_multicolor;
use crate::frontend::web::app::start_web;

pub fn ignite_web_app() {
    write_to_terminal_multicolor("Starting up web application ...").expect("TODO: panic message");
    
    start_web();
}