use std::cell::RefCell;
use std::rc::Rc;

use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
use sourceview5::prelude::BufferExt;
use sourceview5::View;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/zerosofts/BeamerQuickie/window.ui")]
pub struct Window {
    #[template_child]
    pub txt_browse: TemplateChild<gtk::Text>,
    #[template_child]
    pub btn_browse: TemplateChild<gtk::Button>,
    #[template_child]
    pub cb_selectall: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub cb_slidesonly: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub btn_preview: TemplateChild<gtk::Button>,
    #[template_child]
    pub btn_copy: TemplateChild<gtk::Button>,
    #[template_child]
    pub cb_graphics: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub txt_graphics: TemplateChild<gtk::Entry>,
    #[template_child]
    pub btn_graphics: TemplateChild<gtk::Button>,
    #[template_child]
    pub lv_slides: TemplateChild<gtk::ListView>,
    #[template_child]
    pub tv_frame: TemplateChild<View>,
    #[template_child]
    pub buf_frame: TemplateChild<sourceview5::Buffer>,
    pub preamble: RefCell<String>,
    pub slides: RefCell<Option<gio::ListStore>>,
    pub current_slide_content: Rc<RefCell<String>>,
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
        obj.setup_preamble();
        obj.setup_slides();
        obj.setup_callbacks();
        obj.setup_factory();

        obj.imp().buf_frame.set_language(
            sourceview5::LanguageManager::new()
                .language("latex")
                .as_ref(),
        );
        obj.imp().buf_frame.set_style_scheme(
            sourceview5::StyleSchemeManager::new()
                .scheme("Adwaita-dark")
                .as_ref(),
        )
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
