use std::slice::Iter;
use std::string::ToString;
use std::{fs::read_to_string, path::Path};

use crate::slide::{SlideData, SlideType};

pub const BEGIN_FRAME: &'static str = r"\begin{frame}";
pub const END_FRAME: &'static str = r"\end{frame}";
pub const APPENDIX: &'static str = r"\appendix";
pub const BEGIN_DOCUMENT: &'static str = r"\begin{document}";
pub const END_DOCUMENT: &'static str = r"\end{document}";

pub struct BeamerContents {
    preamble: String,
    slides: Vec<SlideData>,
    appendix: Vec<SlideData>,
    unused: Vec<SlideData>,
}

impl ToString for BeamerContents {
    fn to_string(&self) -> String {
        let mut contents = String::new();
        contents.push_str(&self.preamble);
        self.slides.iter().filter(|s| s.include).for_each(|s| {
            contents.push_str(&s.content);
            contents.push_str("\n\n");
        });
        contents.push_str(APPENDIX);
        contents.push_str("\n\n");
        self.appendix.iter().filter(|s| s.include).for_each(|s| {
            contents.push_str(&s.content);
            contents.push_str("\n\n");
        });
        contents.push_str(END_DOCUMENT);
        contents.push_str("\n\n");
        self.unused.iter().filter(|s| s.include).for_each(|s| {
            contents.push_str(&s.content);
            contents.push_str("\n\n");
        });
        contents
    }
}

impl BeamerContents {
    // pub fn new(
    //     preamble: String,
    //     slides: Vec<SlideData>,
    //     appendix: Vec<SlideData>,
    //     unused: Vec<SlideData>,
    // ) -> Self {
    //     let slides = slides
    //         .into_iter()
    //         .map(|mut s| {
    //             s.slidetype = SlideType::Main.to_num();
    //             s
    //         })
    //         .collect();
    //     let appendix = appendix
    //         .into_iter()
    //         .map(|mut s| {
    //             s.slidetype = SlideType::Appendix.to_num();
    //             s
    //         })
    //         .collect();
    //     let unused = unused
    //         .into_iter()
    //         .map(|mut s| {
    //             s.slidetype = SlideType::Unused.to_num();
    //             s
    //         })
    //         .collect();
    //     Self {
    //         preamble,
    //         slides,
    //         appendix,
    //         unused,
    //     }
    // }

    pub fn from_slides(preamble: String, all_slides: Vec<SlideData>) -> BeamerContents {
        let mut slides = Vec::with_capacity(all_slides.len());
        let mut appendix = Vec::with_capacity(all_slides.len());
        let mut unused = Vec::with_capacity(all_slides.len());
        for s in all_slides {
            match SlideType::from_num(s.slidetype).unwrap() {
                SlideType::Main => slides.push(s),
                SlideType::Appendix => appendix.push(s),
                SlideType::Unused => unused.push(s),
            }
        }
        Self {
            preamble,
            slides,
            appendix,
            unused,
        }
    }

    pub fn preamble(&self) -> &str {
        &self.preamble
    }
    pub fn slides(&self) -> Iter<'_, SlideData> {
        self.slides.iter()
    }
    pub fn appendix(&self) -> Iter<'_, SlideData> {
        self.appendix.iter()
    }
    pub fn unused(&self) -> Iter<'_, SlideData> {
        self.unused.iter()
    }

    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<BeamerContents> {
        // everything till the first frame is considered preamle for
        // this use case
        let contents = read_to_string(path)?;
        let p = contents.find(BEGIN_DOCUMENT).unwrap() + BEGIN_DOCUMENT.len();
        let preamble = contents[0..p].to_string();
        let end = contents.find(END_DOCUMENT).unwrap();

        let mut appendix = Vec::new();
        let slides = if let Some(a) = contents[p..].find(APPENDIX) {
            appendix = get_frames(&contents, a + p, end, SlideType::Appendix);
            get_frames(&contents, p, a + p, SlideType::Main)
        } else {
            get_frames(&contents, p, end, SlideType::Main)
        };
        let unused = get_frames(&contents, end, contents.len(), SlideType::Unused);
        Ok(BeamerContents {
            preamble,
            slides,
            appendix,
            unused,
        })
    }
}

fn get_frames(
    contents: &str,
    mut start: usize,
    end: usize,
    slidetype: SlideType,
) -> Vec<SlideData> {
    let mut slides = Vec::new();
    while let Some(p) = contents[start..].find(BEGIN_FRAME) {
        let p = contents[..(p + start)].rfind('\n').unwrap() + 1;
        if p > end {
            return slides;
        }
        let slide_end =
            contents[p..].find(END_FRAME).expect("frame not ended") + p + END_FRAME.len();

        let line_start = contents[..p].chars().filter(|&c| c == '\n').count() + 1;
        let line_end = contents[..slide_end].chars().filter(|&c| c == '\n').count() + 1;

        let s = SlideData::new(
            true,
            line_start as i32,
            line_end as i32,
            contents[p..slide_end].to_string(),
            None,
            slidetype.to_num(),
        );
        slides.push(s);
        start = slide_end;
    }
    slides
}
