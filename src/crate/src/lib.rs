extern crate cfg_if;
extern crate wasm_bindgen;
extern crate zip;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use std::io::Cursor;
use wasm_bindgen::JsValue;
use zip::ZipArchive;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, crate!");
}

#[wasm_bindgen(js_name = getFilenameList)]
pub fn get_filename_list(buf: Vec<u8>) -> Vec<JsValue> {
    let reader = Cursor::new(buf);
    let mut zip = ZipArchive::new(reader).unwrap();
    let mut filename_list: Vec<String> = Vec::new();
    for idx in 0..zip.len() {
        let mut file = zip.by_index(idx).unwrap();
        let name = file.name().to_owned();
        if !name.ends_with("/") {
            filename_list.push(name);
        }
    }
    filename_list.iter().map(|x| JsValue::from_str(x)).collect()
}