use dodrio::{bumpalo, Render, RenderContext};
use log::*;
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[derive(Debug)]
struct SortArray {
    state: Vec<u32>,
}

impl SortArray {
    fn new(vec: Vec<u32>) -> SortArray {
        SortArray { state: vec }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).expect("should initialize logging OK");

    // Get the document's `<body>`.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let vec: Vec<u32> = vec![1, 9, 3, 2, 4, 6, 7, 5, 0, 8];
    let sort_array = SortArray::new(vec);

    info!("sort_array {:#?}", sort_array);
}
