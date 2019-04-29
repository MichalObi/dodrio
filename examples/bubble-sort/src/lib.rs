use dodrio::{bumpalo, Node, Render, RenderContext, Vdom};
use log::*;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct SortArray {
    state: Vec<u32>,
}

impl SortArray {
    fn new(state: Vec<u32>) -> SortArray {
        SortArray { state }
    }

    fn sort(&mut self) {
        let len = self.state.len();

        for i in 0..len - 1 {
            let last = len - i - 1;

            for j in 0..last {
                if self.state[j] > self.state[j + 1] {
                    self.state.swap(j, j + 1)
                }
            }
        }
    }
}

/// The rendering implementation for our SortArray
impl Render for SortArray {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        info!("cx: {:#?}", &cx);

        use dodrio::builder::{button, div, li, text, ul};

        let mut sort_items = bumpalo::collections::Vec::with_capacity_in(self.state.len(), cx.bump);

        for number in &self.state {
            let number_as_string = bumpalo::format!(in cx.bump, "{}", number).into_bump_str();
            sort_items.push(li(&cx).children([text(number_as_string)]).finish())
        }

        // info!("sort_items: {:#?}", sort_items);

        div(&cx)
            .children([
                ul(&cx).children(sort_items).finish(),
                button(&cx)
                    .on("click", |root, vdom, _event| {
                        let sort_array = root.unwrap_mut::<SortArray>();
                        sort_array.sort();
                        vdom.schedule_render(); // should be invoke after sort (?)
                    })
                    .children([text("Sort")])
                    .finish(),
            ])
            .finish()
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

    let vdom = Vdom::new(body.as_ref(), sort_array);

    // info!("vdom for sort {:#?}", vdom);

    // Run the virtual DOM forever and don't unmount it.
    vdom.forget();
}
