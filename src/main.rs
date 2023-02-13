use adw::Application;

use tunnelhound_ui_rs::ui::main_window::MainWindow;
use adw::prelude::*;

fn main() {
    gio::resources_register_include!("tunnelhound.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("com.f-omega.tunnelhound")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = MainWindow::new(app);
    window.present();
}
