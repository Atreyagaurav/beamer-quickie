mod imp;

use glib::Object;
use gtk::glib;
use std::path::PathBuf;

glib::wrapper! {
    pub struct SlideObject(ObjectSubclass<imp::SlideObject>);
}
use itertools::Itertools;

static LABEL_MAX_LEN: usize = 40;
static LABEL_MAX_LINES: usize = 5;

impl SlideObject {
    pub fn new(slide: &SlideData) -> Self {
        Object::builder()
            .property("include", slide.include)
            .property("linestart", slide.linestart)
            .property("lineend", slide.lineend)
            .property("label", slide.label.to_string())
            .property("content", slide.content.to_string())
            .property("image", slide.image.clone())
            .build()
    }
}

#[derive(Default, Debug)]
pub struct SlideData {
    pub include: bool,
    pub linestart: i32,
    pub lineend: i32,
    pub content: String,
    pub label: String,
    pub image: PathBuf,
}

impl SlideData {
    pub fn new(
        include: bool,
        linestart: i32,
        lineend: i32,
        content: String,
        image: Option<PathBuf>,
    ) -> Self {
        Self {
            include,
            linestart,
            lineend,
            label: create_label(&content),
            content,
            image: image.unwrap_or(PathBuf::from("resources/icons/slide.svg")),
        }
    }
}

fn create_label(content: &str) -> String {
    let mut lines: Vec<&str> = Vec::new();
    if content.chars().filter(|c| *c == '\n').count() > LABEL_MAX_LINES {
        let l: Vec<&str> = content.split('\n').collect();
        lines.extend(&l[..(LABEL_MAX_LINES - 2)]);
        lines.push("⋮");
        lines.push(l[l.len() - 1]);
    } else {
        lines.extend(content.split('\n'));
    }
    lines.into_iter().map(truncate_line).join("\n")
}

fn truncate_line(line: &str) -> String {
    if line.chars().count() > LABEL_MAX_LEN {
        format!("{}…", line.chars().take(LABEL_MAX_LEN - 1).join(""))
    } else {
        line.to_string()
    }
}
