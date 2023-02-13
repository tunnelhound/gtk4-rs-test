use adw::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}


impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        glib::Object::builder().property("application", app).build()
    }
}
