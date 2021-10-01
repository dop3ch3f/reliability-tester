use gtk::prelude::*;
use gtk::{HeaderBar,StackSidebar, Box, Widget, Orientation, WindowControls, PackType};
use crate::frontend::desktop::components::navigation_bar::navigation_bar;
use crate::frontend::desktop::components::sidebar::sidebar;

pub fn default_layout() -> Box {
    let template = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    //let navigation_bar= navigation_bar();

   let stack_sidebar = sidebar();

    //template.prepend(&navigation_bar);

   template.prepend(&stack_sidebar);

    template
}