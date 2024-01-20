mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

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
        // // Get state
        // let cb_slide = self.imp().cb_slide.get();
        let lb_slide = self.imp().lb_slide.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // // Bind `task_object.completed` to `task_row.completed_button.active`
        // let completed_button_binding = task_object
        //     .bind_property("completed", &completed_button, "active")
        //     .bidirectional()
        //     .sync_create()
        //     .build();
        // // Save binding
        // bindings.push(completed_button_binding);

        // Bind `task_object.content` to `task_row.content_label.label`
        let label_binding = slide_object
            .bind_property("label", &lb_slide, "label")
            .sync_create()
            .build();
        // // Save binding
        bindings.push(label_binding);

        // // Bind `task_object.completed` to `task_row.content_label.attributes`
        // let content_label_binding = task_object
        //     .bind_property("completed", &content_label, "attributes")
        //     .sync_create()
        //     .transform_to(|_, active| {
        //         let attribute_list = AttrList::new();
        //         if active {
        //             // If "active" is true, content of the label will be strikethrough
        //             let attribute = AttrInt::new_strikethrough(true);
        //             attribute_list.insert(attribute);
        //         }
        //         Some(attribute_list.to_value())
        //     })
        //     .build();
        // // Save binding
        // bindings.push(content_label_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
