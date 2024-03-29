use libc::c_int;
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

pub struct Scanner {
    inner: synctex_sys::synctex_scanner_p,
}

// struct Node {
//     inner: synctex_sys::synctex_node_p,
// }

impl Scanner {
    pub fn from_output(output: &Path, dir: Option<&Path>) -> Option<Self> {
        let dir = dir.unwrap_or(output.parent()?);
        let output = CString::new(output.to_string_lossy().as_ref()).unwrap();
        let dir = CString::new(dir.to_string_lossy().as_ref()).unwrap();
        let parse: c_int = 1;
        let inner = unsafe {
            synctex_sys::synctex_scanner_new_with_output_file(
                output.into_raw(),
                dir.into_raw(),
                parse,
            )
        };
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub fn edit_query(&self, page: i32, x: f32, y: f32) -> (PathBuf, i32) {
        unsafe {
            synctex_sys::synctex_edit_query(self.inner, page, x, y);
            let node = synctex_sys::synctex_scanner_next_result(self.inner);
            let input = synctex_sys::synctex_scanner_get_name(
                self.inner,
                synctex_sys::synctex_node_tag(node),
            );
            let line = synctex_sys::synctex_node_line(node);

            let c_str = CStr::from_ptr(input);
            let input: PathBuf = c_str.to_string_lossy().into_owned().into();

            (input, line)
        }
    }

    /// Get the line in the latex source for each pages in the pdf
    pub fn get_lines(&self, pages: &[i32]) -> Vec<(PathBuf, i32)> {
        pages
            .iter()
            .map(|&p| self.edit_query(p, 1.0, 1.0))
            .collect()
    }
}
