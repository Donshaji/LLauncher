use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};

fn main() {
    let app = Application::builder()
        .application_id("dev.llauncher.LLauncher")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("LLauncher")
            .default_width(432)
            .default_height(900)
            .child(&Label::new(Some("LLauncher is ready")))
            .build();

        window.present();
    });

    app.run();
}
