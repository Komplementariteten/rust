use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, SearchBar, FileChooserDialog, FileFilter, Window, FileChooserAction, SignalAction, Dialog, ResponseType};
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use crate::file_chooser::FileChooserButton;

/// The private struct, which can hold widgets and other data.
#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtkpwsafe/window.ui")]
pub struct StartWindow {
    #[template_child]
    pub file_filter: TemplateChild<FileFilter>
}


#[glib::object_subclass]
impl ObjectSubclass for StartWindow {
    const NAME: &'static str = "StartWindow";
    type Type = super::StartWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        FileChooserButton::ensure_type();
        klass.bind_template();
        klass.bind_template_callbacks();
        //UtilityCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl StartWindow {
    #[template_callback]
    fn choose_file(&self, btn: &FileChooserButton) {
        /* let fs = FileChooserDialog::builder().title("Choose PwSafe store").filter(&self.file_filter).action(FileChooserAction::Open).build(); */
        let fs = FileChooserDialog::new(Some("Choose PwSafe store"), Some(self), FileChooserAction::Open, &[("Open", ResponseType::Accept), ("Cancel", ResponseType::Cancel)]);
        fs.connect_response(| fc, resp | {
            if resp == ResponseType::Accept {
                println!("accept")
            }
        });
        fs.show_all();
        println!("hello");
    }
}

impl ObjectImpl for StartWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for StartWindow {}
impl WindowImpl for StartWindow {}
impl ApplicationWindowImpl for StartWindow {}