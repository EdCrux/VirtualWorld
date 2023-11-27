
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;
use super::primitives::structs::Segment;
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Graph<'a,> {
    pub segments : &'a mut Vec<Segment>
}

impl<'a> Graph<'a> {

    pub fn new (segments: &'a mut Vec<Segment> ) -> Graph<'a> {
        return Graph {
            segments : segments
        }
    }

    pub fn draw(&self, ctx : &CanvasRenderingContext2d) -> () {
        for seg in &*self.segments {
            let color = JsValue::from_str("black");
            let size = 10.0;
            // Draw the segment.
            seg.draw(&ctx, 2.0, &color);
            
            console_log!("Drawing the graph");
            seg.p1.draw(&ctx, size, &color);
            seg.p2.draw(&ctx, size, &color);
        }
    }
}