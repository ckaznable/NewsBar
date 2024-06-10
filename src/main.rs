use gdk::glib;
use gtk::{prelude::*, Application};
use app::App;

mod app;
mod style;

const APP_ID: &str = "NewsBar";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let app = App::new(window);
        app.init();
        app.present();
    });

    app.run()
}
