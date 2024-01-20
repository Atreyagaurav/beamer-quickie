mod imp;

use glib::Object;
use gtk::glib;
use itertools::Itertools;

glib::wrapper! {
    pub struct SlideObject(ObjectSubclass<imp::SlideObject>);
}

impl SlideObject {
    pub fn new(content: String) -> Self {
        let label = content.replace('\n', " ↵ ");
        Object::builder()
            .property(
                "label",
                if label.chars().count() > 40 {
                    format!("{}...", &label.chars().take(37).join(""))
                } else {
                    label
                },
            )
            .property("content", content)
            .build()
    }
}

#[derive(Default)]
pub struct SlideData {
    pub content: String,
    pub label: String,
}
