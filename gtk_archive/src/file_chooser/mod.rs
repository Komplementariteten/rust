mod imp;

use gtk::glib;
use gtk::glib::Object;

glib::wrapper! {
    pub struct FileChooserButton(ObjectSubclass<imp::FileChooserButton>) @extends gtk::Button, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl FileChooserButton {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `FileChooserButton`.")
    }
}

impl Default for FileChooserButton{
    fn default() -> Self {
        Self::new()
    }
}