use std::cell::RefCell;
use std::path::PathBuf;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::SlideData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::SlideObject)]
pub struct SlideObject {
    #[property(name = "include", get, set, type = bool, member = include)]
    #[property(name = "linestart", get, set, type = i32, member = linestart)]
    #[property(name = "lineend", get, set, type = i32, member = lineend)]
    #[property(name = "content", get, set, type = String, member = content)]
    #[property(name = "label", get, set, type = String, member = label)]
    #[property(name = "image", get, set, type = PathBuf, member = image)]
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
