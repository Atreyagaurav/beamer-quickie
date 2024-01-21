use std::{collections::HashMap, path::Path};

use poppler;

pub fn frames_pages(path: &Path) -> Vec<usize> {
    let file = format!("file:{}", path.to_string_lossy());
    let file = poppler::Document::from_file(&file, None).unwrap();
    let npage = file.n_pages();
    let labels: Vec<String> = (0..npage)
        .map(|i| {
            file.page(i)
                .unwrap()
                .label()
                .map(|l| l.to_string())
                .unwrap_or(i.to_string())
        })
        .collect();
    let mut frames: HashMap<String, usize> = HashMap::new();
    for (i, l) in labels.iter().enumerate() {
        if !frames.contains_key(l) {
            frames.insert(l.to_string(), i);
        }
    }
    let mut pages: Vec<usize> = frames.values().map(|v| *v).collect();
    pages.sort();
    pages
}
