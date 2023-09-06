use gtk::prelude::*;
use gtk::{Application};

use gtk::glib::{Char, OptionFlags, OptionArg};

use crate::frontend::desktop::screens::home::build_home;


pub fn start_desktop() {
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

    app.connect_activate(build_home);

    // Run the application
    app.run();
}