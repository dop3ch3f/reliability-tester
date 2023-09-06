use gtk::prelude::*;
use gtk::{Box,Orientation, PackType, WindowControls};

pub fn _navigation_bar() -> Box {
    let template = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    let left_window_control = WindowControls::builder()
        .side(PackType::Start)
        .build();
    let right_window_control = WindowControls::builder()
        .side(PackType::End)
        .build();

    template.prepend(&left_window_control);
    template.prepend(&right_window_control);

    template
}