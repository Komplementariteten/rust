mod start;
mod appwindow;
mod file_chooser;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, gio};

const APP_ID: &str = "org.gtkpwsafe.HelloWorld1";
const APP_TITLE: &str = "Gtk-Pwsafe";

fn main() {
    gio::resources_register_include!("start.gresource").expect("Failed to register resources");
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        // let win = example_win::ExApplicationWindow::new(app);
        let win = start::StartWindow::new(app);
        win.show();
    });

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    //let action_panel = start::prepare_main_panel();
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        // .child(&action_panel)
        .build();

    // Present window
    window.present();
}