use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};



use crate::frontend::desktop::components::buttons::default_button;
use crate::frontend::desktop::layouts::default::default_layout;
use std::rc::Rc;
use std::cell::Cell;

pub fn build_home(app: &Application) {
    // Create homepage window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_height(1000)
        .default_width(1000)
        .title("Reliability Tester")
        .build();

    let _home_button = default_button("Click Me!", move |b| {
        b.set_label("Welcome to Reliability Tester");
    });

    // Reference-counted object with inner-mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks, when a button is clicked `number` will be changed
    let _number_copy_1 = number.clone();

    let _increase_button = default_button("+", |_b| {});
    let _decrease_button = default_button("-", |_b| {});

    let default_layout: gtk::Box = default_layout().object("box").expect("Could not get object `box` from builder.");

    window.set_child(Some(&default_layout));
    window.present();
}