mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::slide::SlideObject;

glib::wrapper! {
    pub struct SlideRow(ObjectSubclass<imp::SlideRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for SlideRow {
    fn default() -> Self {
        Self::new()
    }
}

impl SlideRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, slide_object: &SlideObject) {
        let lb_slide = self.imp().lb_slide.get();
        let cb_slide = self.imp().cb_slide.get();
        let dd_slide = self.imp().dd_slide.get();
        let img_slide = self.imp().img_slide.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let include_binding = slide_object
            .bind_property("include", &cb_slide, "active")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(include_binding);

        let image_binding = slide_object
            .bind_property("image", &img_slide, "file")
            .sync_create()
            .build();
        bindings.push(image_binding);

        let label_binding = slide_object
            .bind_property("label", &lb_slide, "label")
            .sync_create()
            .build();
        bindings.push(label_binding);

        let slidetype_binding = slide_object
            .bind_property("slidetype", &dd_slide, "selected")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(slidetype_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
