use lazy_static::lazy_static;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::slice::Iter;
use std::string::ToString;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub const BEGIN_FRAME: &'static str = r"\begin{frame}";
pub const END_FRAME: &'static str = r"\end{frame}";
pub const APPENDIX: &'static str = r"\appendix";
pub const BEGIN_DOCUMENT: &'static str = r"\begin{document}";
pub const END_DOCUMENT: &'static str = r"\end{document}";

lazy_static! {
    // todo: think about handling windows
    pub static ref CACHE: PathBuf = PathBuf::from("/tmp/.cache/beamer-quickie");
}

pub struct BeamerContents {
    preamble: String,
    slides: Vec<String>,
    appendix: Vec<String>,
    unused: Vec<String>,
}

impl ToString for BeamerContents {
    fn to_string(&self) -> String {
        let mut contents = String::new();
        contents.push_str(&self.preamble);
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

impl BeamerContents {
    pub fn single_frame_tex(&self, frame: &str) -> String {
        let mut contents = String::new();
        contents.push_str(&self.preamble);
        contents.push_str(frame);
        contents.push('\n');
        contents.push_str(END_DOCUMENT);
        contents.push('\n');
        contents
    }

    pub fn preamble(&self) -> &str {
        &self.preamble
    }
    pub fn slides(&self) -> Iter<'_, String> {
        self.slides.iter()
    }
    pub fn appendix(&self) -> Iter<'_, String> {
        self.appendix.iter()
    }
    pub fn unused(&self) -> Iter<'_, String> {
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
            appendix = get_frames(&contents, a + p, end);
            get_frames(&contents, p, a + p)
        } else {
            get_frames(&contents, p, end)
        };
        let unused = get_frames(&contents, end, contents.len());
        Ok(BeamerContents {
            preamble,
            slides,
            appendix,
            unused,
        })
    }
}

fn contents_hash(contents: &str) -> u64 {
    let mut s = DefaultHasher::new();
    contents.hash(&mut s);
    s.finish()
}

fn get_frames(contents: &str, mut start: usize, end: usize) -> Vec<String> {
    let mut slides = Vec::new();
    while let Some(p) = contents[start..].find(BEGIN_FRAME) {
        let p = p + start;
        if p > end {
            return slides;
        }
        let slide_end =
            contents[p..].find(END_FRAME).expect("frame not ended") + p + END_FRAME.len();
        slides.push(contents[p..slide_end].to_string());
        start = slide_end;
    }
    slides
}

pub fn thumbnail(cache_dir: &Path, contents: &str) -> PathBuf {
    let hash = contents_hash(contents);
    let fname = cache_dir.join(hash.to_string()).join("1.png");
    if !fname.exists() {
        println!("Create: {:?}", fname);
    }
    fname
}
