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
            .property("slidetype", slide.slidetype)
            .build()
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum SlideType {
    #[default]
    Main,
    Appendix,
    Unused,
}

impl ToString for SlideType {
    fn to_string(&self) -> String {
        match self {
            SlideType::Main => "Main",
            SlideType::Appendix => "Appendix",
            SlideType::Unused => "Unused",
        }
        .to_string()
    }
}

// impl glib::value::ToValue for SlideType {
//     fn to_value(&self) -> glib::value::Value {
//         match self {
//             SlideType::Main => glib::value::Value::from(0),
//             SlideType::Appendix => glib::value::Value::from(1),
//             SlideType::Unused => glib::value::Value::from(2),
//         }
//     }

//     fn value_type(&self) -> glib::Type {
//         glib::Type::ENUM
//     }
// }

// impl glib::HasParamSpec for SlideType {
//     type ParamSpec = u32;
//     type SetValue = Self;
//     type BuilderFn = fn(&str) -> u32;

//     fn param_spec_builder() -> Self::BuilderFn {
//         SlideType::str_to_num
//     }
// }

impl SlideType {
    pub fn from_num(n: u8) -> Result<Self, ()> {
        match n {
            0 => Ok(SlideType::Main),
            1 => Ok(SlideType::Appendix),
            2 => Ok(SlideType::Unused),
            _ => Err(()),
        }
    }

    pub fn to_num(&self) -> u8 {
        match self {
            SlideType::Main => 0,
            SlideType::Appendix => 1,
            SlideType::Unused => 2,
        }
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
    pub slidetype: u8,
}

impl SlideData {
    pub fn new(
        include: bool,
        linestart: i32,
        lineend: i32,
        content: String,
        image: Option<PathBuf>,
        slidetype: u8,
    ) -> Self {
        Self {
            include,
            linestart,
            lineend,
            label: create_label(&content),
            content,
            image: image.unwrap_or(PathBuf::from("resources/icons/slide.svg")),
            slidetype,
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
