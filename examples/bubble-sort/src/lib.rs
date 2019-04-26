use dodrio::{bumpalo, Render, RenderContext};
use log::*;
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[derive(Debug)]
struct sortArray {
    state: Vec<u32>,
}

impl sortArray {
    fn new(vec: &Vec<u32>) -> sortArray {
        sortArray {
            state: vec.to_vec(),
        }
    }

    fn shuffle() {}
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).expect("should initialize logging OK");

    // Get the document's `<body>`.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let vec: Vec<u32> = (0..10).collect();

    info!("vec {:#?}", vec);

    let sort_array = sortArray::new(&vec);

    info!("sort_array {:#?}", sort_array);
}
