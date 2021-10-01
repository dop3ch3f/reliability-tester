use gtk::prelude::*;
use gtk::{HeaderBar,StackSidebar, Box, Widget, Orientation, WindowControls, PackType, Stack};

pub fn sidebar() -> Box {
    let template = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let stack = Stack::builder().build();

    let stack_sidebar = StackSidebar::builder().build();

    template.prepend(&stack_sidebar);

    template
}

