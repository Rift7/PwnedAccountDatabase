extern crate sled;
extern crate gtk;

use std::io;
use gtk::prelude::*;
use gtk::{Builder, Entry, TextView, Window};

fn main() {
    // Initialize GTK
    gtk::init().unwrap();

    // Load the GTK builder file
    let builder = Builder::new_from_string(include_str!("search_window.glade"));

    // Get a reference to the search entry, output text view, and window
    let search_entry: Entry = builder.get_object("search_entry").unwrap();
    let output_text_view: TextView = builder.get_object("output_text_view").unwrap();
    let window: Window = builder.get_object("window").unwrap();

    // Connect the search entry's "changed" signal to the search handler
    search_entry.connect_changed(move |entry| {
        let query = entry.get_text().unwrap();
        search_database(&query, &output_text_view);
    });

    // Show the window and start the GTK main loop
    window.show_all();
    gtk::main();
}
