use gtk::prelude::*;
use gtk::{Button};

pub fn default_button(
    label: &str,
    callback: for<'r> fn(&'r Button) -> ()
) -> Button  {

    let button = Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect callback
    button.connect_clicked(callback);

    button
}