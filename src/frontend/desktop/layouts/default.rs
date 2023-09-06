
use gtk::{Builder};



pub fn default_layout() -> Builder {
    let builder = Builder::from_string(include_str!("default.ui"));
   //  let template = Box::builder()
   //      .orientation(Orientation::Vertical)
   //      .build();
   //
   //  //let navigation_bar= navigation_bar();
   //
   // let stack_sidebar = sidebar();
   //
   //  //template.prepend(&navigation_bar);
   //
   // template.prepend(&stack_sidebar);
   //
   //  template

    builder
}