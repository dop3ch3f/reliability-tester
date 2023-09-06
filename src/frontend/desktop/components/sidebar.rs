use gtk::prelude::*;
use gtk::{StackSidebar, Box, Orientation, Stack};

pub fn _sidebar() -> Box {
    let template = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let _stack = Stack::builder().build();

    let stack_sidebar = StackSidebar::builder().build();

    template.prepend(&stack_sidebar);

    template
}

