use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button, SearchBar};

/// The private struct, which can hold widgets and other data.
#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "window.ui")]
pub struct MainWindow {

}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        //UtilityCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        obj.init_label();
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}