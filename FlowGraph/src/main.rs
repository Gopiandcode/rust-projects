#![feature(extern_prelude, try_from)]
extern crate gtk;
extern crate gdk;
extern crate cairo;
#[macro_use] extern crate dependent_view;

mod color_scheme;
mod types;
mod render_window;
mod color;
mod drawing_context;
mod component_renderer;
mod component_ui;

use render_window::RenderWindow;


use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;
use component_ui::*;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt};
use gdk::EventMask;

fn main() {
    gtk::init();


    let window = Window::new(gtk::WindowType::Toplevel);

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        // main_quit ends the gtk event loop, thus prompting our
        // app to close - there's no need to run the default 
        // handler
        gtk::Inhibit(false) 
    });
    let renderer = ComponentRenderer::<ComponentUI>::new_component_renderer(ComponentUI::default());
    window.add(&renderer.get_drawing_area());

    window.show_all();

    gtk::main();
}



