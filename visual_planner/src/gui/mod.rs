pub mod app;
pub mod manager;

pub use self::app::App;
use renderer::{RenderWindow, StyleScheme};

use gtk::{
    Window,          // container in which rest of application will be kept
    WidgetExt,       // required to call show_all() on the window
    main,            // used to start the gtk application
    init as gtk_init // must be called before any other calls to gtk frameworks
};

use std::convert::AsRef;
use gdk;

pub fn init() {
    gdk::init();
    gdk::threads_init();
    gdk::threads_enter();
    // TODO: Check: http://antipastohw.pbworks.com/w/page/27640084/BeagleBoard%20GTK%20Development#PongBall
    
    if let Err(err) = gtk_init() {
        panic!("ERROR: While initializing gtk - {}", err);    
    };
    
}

