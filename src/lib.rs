
use js_sys::Error;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod math;
use math::graph::Graph;
use math::primitives::structs::{Point, Segment};


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


fn get_context() -> Result<CanvasRenderingContext2d,Error> { 

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

    Ok(context)
}

#[wasm_bindgen(start)]
fn start() {
    let ctx = get_context().unwrap();
    
    let segment = Segment::new(
        Point::new(100.0, 100.0), 
        Point::new(200.0, 200.0)
    );
    
    let mut segments = vec![
        segment
    ];

    let graph = Graph::new(&mut segments);
    graph.draw(&ctx)
}
