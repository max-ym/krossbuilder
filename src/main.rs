extern crate cairo;
extern crate gtk;
extern crate gio;
use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use gio::{APPLICATION_FLAGS_NONE, ApplicationExt};

/// All stuff related to page content.
mod page;

/// Drawing tools. Allow to draw page content.
mod draw;

const MAIN_WINDOW_TITLE : &'static str = "Krossbuilder";
const APP_ID            : &'static str = "com.krossbuilder";

const MAIN_WINDOW_GLADE : &'static str = include_str!("mainwindow.glade");

fn main() {
    // Initialize GTK.
    let gtk_init_status = gtk::Application::new(
        Some(APP_ID),
        APPLICATION_FLAGS_NONE);
    if gtk_init_status.is_err() {
        println!("Failed to create GTK window!");
        return;
    }
    let application = gtk_init_status.unwrap();
    application.connect_activate(|_| {
        let builder = gtk::Builder::new_from_string(MAIN_WINDOW_GLADE);
        let main_window: Window
                = builder.get_object("applicationwindow1").unwrap();
        main_window.set_title(MAIN_WINDOW_TITLE);
        main_window.show_all();
        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let area: DrawingArea
                = builder.get_object("drawingarea1").unwrap();
        area.connect_draw(draw::draw_fn);
    });

    application.run(&[]);

    // Run main loop
    gtk::main();
}
