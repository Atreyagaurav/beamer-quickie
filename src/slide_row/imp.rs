use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{self, glib, CompositeTemplate};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/zerosofts/BeamerQuickie/slide.ui")]
pub struct SlideRow {
    #[template_child]
    pub cb_slide: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub lb_slide: TemplateChild<gtk::Label>,
    #[template_child]
    pub dd_slide: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub img_slide: TemplateChild<gtk::Image>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SlideRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SlideRow";
    type Type = super::SlideRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for SlideRow {}

// Trait shared by all widgets
impl WidgetImpl for SlideRow {}

// Trait shared by all boxes
impl BoxImpl for SlideRow {}
