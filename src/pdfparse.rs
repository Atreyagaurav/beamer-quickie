use cairo;
use lazy_static::lazy_static;
use poppler;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
lazy_static! {
    pub static ref CACHE_DIR: PathBuf = {
        let d = std::env::temp_dir().join("beamer-quickie");
        if !d.exists() {
            std::fs::create_dir(&d).unwrap();
        }
        d
    };
}

pub fn pdf_pages_count(path: &Path) -> i32 {
    let file = format!("file:{}", path.to_string_lossy());
    let file = poppler::Document::from_file(&file, None).unwrap();
    file.n_pages()
}

pub fn get_thumbnail(path: &Path, page: usize) -> PathBuf {
    let mut h = DefaultHasher::new();
    path.hash(&mut h);
    let hash = format!("{}-{}", h.finish(), page);
    let thumb = CACHE_DIR.join(hash).with_extension("svg");
    if !thumb.exists() || older_than(&thumb, path) {
        generate_thumb(path, page, &thumb);
    }
    thumb
}

fn older_than(thumb: &Path, pdf: &Path) -> bool {
    let pdfmeta = File::open(pdf).unwrap().metadata().unwrap();
    let pdftime = pdfmeta
        .modified()
        .ok()
        .unwrap_or(pdfmeta.created().unwrap());
    let thumbmeta = File::open(thumb).unwrap().metadata().unwrap();
    let thumbtime = thumbmeta
        .modified()
        .ok()
        .unwrap_or(thumbmeta.created().unwrap());
    pdftime > thumbtime
}

fn generate_thumb(pdf: &Path, page: usize, path: &Path) {
    let file = format!("file:{}", pdf.to_string_lossy());
    let file = poppler::Document::from_file(&file, None).unwrap();
    let page = file.page(page as i32).unwrap();
    let (w, h) = page.size();
    let (ws, hs) = {
        if w > h {
            (128.0, (h * 128.0 / w))
        } else {
            ((w * 128.0 / h), 128.0)
        }
    };
    let mut surface = cairo::SvgSurface::new(ws, hs, Some(path)).unwrap();
    surface.set_document_unit(cairo::SvgUnit::Px);
    surface.set_device_scale(ws / w, hs / h);

    let ctx = cairo::Context::new(&mut surface).unwrap();
    page.render(&ctx);
    surface.finish();
    surface.flush();
}

pub fn frames_pages(path: &Path) -> Vec<usize> {
    let file = format!("file:{}", path.to_string_lossy());
    let file = poppler::Document::from_file(&file, None).unwrap();
    let npage = file.n_pages();
    let mut labels = (0..npage).map(|i| {
        file.page(i)
            .unwrap()
            .label()
            .map(|l| l.to_string())
            .unwrap_or(i.to_string())
    });
    let mut pages: Vec<usize> = Vec::new();

    let mut prev = if let Some(l) = labels.next() {
        pages.push(0);
        l
    } else {
        return pages;
    };
    let mut i = 1;
    for l in labels {
        if prev != l {
            pages.push(i);
            prev = l;
        }

        i = i + 1;
    }
    pages
}
