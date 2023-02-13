use gio::subclass::prelude::ObjectSubclass;
use glib::subclass::InitializingObject;
use gtk::{TemplateChild, CompositeTemplate};
use adw::subclass::prelude::*;
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/f-omega/tunnelhound/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub stack: TemplateChild<adw::ViewStack>,
    #[template_child]
    pub devices: TemplateChild<adw::ViewStackPage>,
    #[template_child]
    pub device_list: TemplateChild<gtk::ListView>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ApplicationWindowImpl for MainWindow { }
impl WindowImpl for MainWindow { }
impl WidgetImpl for MainWindow { }

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl AdwApplicationWindowImpl for MainWindow {
}
