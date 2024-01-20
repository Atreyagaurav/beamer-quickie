use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::SlideData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::SlideObject)]
pub struct SlideObject {
    #[property(name = "content", get, set, type = String, member = content)]
    #[property(name = "label", get, set, type = String, member = label)]
    pub data: RefCell<SlideData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SlideObject {
    const NAME: &'static str = "SlideObject";
    type Type = super::SlideObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for SlideObject {}
