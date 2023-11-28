
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;
use super::primitives::structs::{Point, Segment};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Graph<'a,> {
    pub points :  &'a mut Vec<Point>,
    pub segments : &'a mut Vec<Segment<'a>>
}

impl<'a> Graph<'a> {

    pub fn new (points: &'a mut Vec<Point>, segments: &'a mut Vec<Segment<'a>> ) -> Graph<'a> {
        return Graph {
            points : points,
            segments : segments
        }
    }

    pub fn add_point(&mut self, point : Point) -> () {
        self.points.push(point);
    }

    pub fn contains_point(&mut self, point : Point) -> bool {
        return self.points.into_iter().any(|p| p.equals(&point))
    }

    pub fn add_segment(&mut self, segment : Segment<'a>) -> () {
        self.segments.push(segment);
    }

    pub fn contains_segments(&mut self, seg : Segment) -> bool {
        return self.segments.into_iter().any(|s| s.equals(&seg))
    }

    pub fn try_add_point(&mut self, point : Point) -> bool {
        
        if !self.contains_point(point.clone()) { 
            self.add_point(point.clone());
            return true;
        }

        return false;
    }

    pub fn get_segments_with_point(&self, point : Point) -> Vec<Segment<'_>> {
        
        let mut segs = Vec::new();
        for seg in &*self.segments {
            if seg.includes(&point) {
                segs.push(seg.clone())
            }
        }

        return segs;
    }

    pub fn try_add_segment(&mut self, seg : Segment<'a>) -> bool {
        let seg_p2 = seg.p2.clone();
        if !self.contains_segments(seg.clone()) && !seg.p1.equals(&seg_p2) {
            self.add_segment(seg);
            return true;
        }

        return false;

    }



    pub fn draw(&self, ctx : &CanvasRenderingContext2d) -> () {

        for seg in &*self.segments {
            let color = JsValue::from_str("black");
            // Draw the segment.
            seg.draw(&ctx, 2.0, &color);
        }

        for point in &*self.points {
            let color = JsValue::from_str("black");
            let size = 10.0;
            point.draw(&ctx, size, &color)
        }
    }
}