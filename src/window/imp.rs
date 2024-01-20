use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, Entry, ListView};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/zerosofts/BeamerQuickie/window.ui")]
pub struct Window {
    #[template_child]
    pub txt_browse: TemplateChild<gtk::Text>,
    #[template_child]
    pub btn_browse: TemplateChild<gtk::Button>,
    #[template_child]
    pub lv_slides: TemplateChild<gtk::ListView>,
    pub slides: RefCell<Option<gio::ListStore>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "BeamerQuickieWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj = self.obj();
        obj.setup_slides();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}