extern crate gtk;
extern crate gio;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use gio::APPLICATION_FLAGS_NONE;

const MAIN_WINDOW_TITLE : &'static str = "Krossbuilder";
const APP_ID            : &'static str = "com.krossbuilder";

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
        let main_window = Window::new(WindowType::Toplevel);
        main_window.set_title(MAIN_WINDOW_TITLE);
        main_window.set_default_size(800, 600);
        main_window.show_all();
        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    });
    application.run(0, &[]);

    // Run main loop
    gtk::main();
}
