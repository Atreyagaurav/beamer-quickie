mod pdfparse;
mod slide;
mod slide_row;
mod synctex;
mod texparse;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("beamer-quickie.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.zerosofts.BeamerQuickie")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}
