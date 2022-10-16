use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, FileFilter};

#[derive(Debug, Default)]
pub struct FileChooserButton {
}

#[glib::object_subclass]
impl ObjectSubclass for FileChooserButton {
    const NAME: &'static str = "MyFileChooserButton";
    type Type = super::FileChooserButton;
    type ParentType = gtk::Button;
}
impl ObjectImpl for FileChooserButton {
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
}

impl WidgetImpl for FileChooserButton {
}
impl ButtonImpl for FileChooserButton {
}