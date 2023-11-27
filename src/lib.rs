
use wasm_bindgen::prelude::*;
mod primitives;
use primitives::structs::{Point , Segment};
mod graph;
use graph::Graph;


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_context() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("virtualWorld").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
}

#[wasm_bindgen(start)]
fn start() {
    let ctx = get_context();
    console_log!("from the rust code")

    primitives.

}
