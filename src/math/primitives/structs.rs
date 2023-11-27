use std::f64;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone)]
pub struct Point {
    pub x : f64,
    pub y : f64 
}

impl Point  {

    pub fn new( x : f64, y : f64) -> Point {
        return Point {
            x : x,
            y : y
        }
    }

    pub fn equals(&self, point : &Point) -> bool {
        return self.x == point.x && self.y == point.y;
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, size : f64, color : &JsValue) -> () {
        
        let rad = size / 2 as f64;

        console_log!("{:?}",color);
        console_log!("X: {}", self.x);
        console_log!("Y: {}", self.y);
        console_log!("Rad: {}", rad);

        // Draw the outer circle.
        ctx.begin_path();
        ctx.set_fill_style(color);
        ctx
            .arc(self.x, self.y, rad, 0.0,f64::consts::PI * 2.0)
            .unwrap();
        ctx.fill();
        
    }
}

pub struct Segment {
    pub p1 : Point,
    pub p2 : Point 
}

impl Segment  {
    pub fn new(p1 : Point, p2 : Point) -> Segment {
        return Segment {
            p1: p1,
            p2: p2
        }
    }


    pub fn equals (&self, seg : &Segment) -> bool {
        return self.includes(&seg.p1) || self.includes(&seg.p2)
    }

    pub fn includes(&self, point : &Point) -> bool {
        return self.p1.equals(point) || self.p2.equals(point)
    }

    pub fn draw(&self, ctx : &CanvasRenderingContext2d, width : f64, color : &JsValue) -> () {
        ctx.begin_path();
        ctx.set_line_width(width);
        ctx.set_stroke_style(color);
        ctx.move_to(self.p1.x, self.p1.y);
        ctx.line_to(self.p2.x, self.p2.y);
        ctx.stroke();
    }

}
