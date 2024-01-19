use std::string::ToString;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub const BEGIN_FRAME: &'static str = r"\begin{frame}";
pub const END_FRAME: &'static str = r"\end{frame}";
pub const APPENDIX: &'static str = r"\appendix";
pub const END_DOCUMENT: &'static str = r"\end{document}";

pub struct BeamerContents<'a> {
    preamble: &'a str,
    slides: Vec<&'a str>,
    appendix: Vec<&'a str>,
    unused: Vec<&'a str>,
}

impl<'a> ToString for BeamerContents<'a> {
    fn to_string(&self) -> String {
        let mut contents = String::new();
        contents.push_str(self.preamble);
        self.slides.iter().for_each(|s| {
            contents.push_str(s);
            contents.push('\n');
        });
        contents.push_str(APPENDIX);
        contents.push('\n');
        self.appendix.iter().for_each(|s| {
            contents.push_str(s);
            contents.push('\n');
        });
        contents.push_str(END_DOCUMENT);
        contents.push('\n');
        self.unused.iter().for_each(|s| {
            contents.push_str(s);
            contents.push('\n');
        });
        contents
    }
}

pub struct BeamerFile {
    path: PathBuf,
    contents: String,
}

impl BeamerFile {
    pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            contents: read_to_string(path)?,
        })
    }

    pub fn parse(&self) -> BeamerContents {
        // everything till the first frame is considered preamle for
        // this use case
        let p = self.contents.find(BEGIN_FRAME).unwrap();
        let preamble = &self.contents[0..p];
        let end = self.contents.find(END_DOCUMENT).unwrap();

        let mut appendix = Vec::new();
        let slides = if let Some(a) = self.contents[p..].find(APPENDIX) {
            appendix = self.get_frames(a + p, end);
            self.get_frames(p, a + p)
        } else {
            self.get_frames(p, end)
        };
        let unused = self.get_frames(end, self.contents.len());
        BeamerContents {
            preamble,
            slides,
            appendix,
            unused,
        }
    }

    fn get_frames(&self, mut start: usize, end: usize) -> Vec<&str> {
        let mut slides = Vec::new();
        while let Some(p) = self.contents[start..].find(BEGIN_FRAME) {
            let p = p + start;
            if p > end {
                return slides;
            }
            let slide_end =
                self.contents[p..].find(END_FRAME).expect("frame not ended") + p + END_FRAME.len();
            slides.push(&self.contents[p..slide_end]);
            start = slide_end;
        }
        slides
    }
}
