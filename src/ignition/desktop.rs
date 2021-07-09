use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::gio::{ApplicationFlags};
use gtk::glib::{Char, OptionFlags, OptionArg};
use crate::util::write_to_terminal_multicolor;
use std::str::FromStr;

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_height(1000)
        .default_width(1000)
        .title("Reliability Tester")
        .build();

    window.present();
}

pub fn ignite_desktop() {
    write_to_terminal_multicolor("Starting up desktop application ...");

    // Create a new application
    let app = Application::builder()
        .application_id("org.reliability.ifeanyi")
        .build();

    app.add_main_option(
        "mode",
        Char::from('m' as u8),
        OptionFlags::IN_MAIN,
        OptionArg::String,
        "A flag just to cater for the main application bootstrap flag (mode)",
        Some("")
    );

   // app.connect_handle_local_options()

    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

