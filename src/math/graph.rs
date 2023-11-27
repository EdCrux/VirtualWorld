use primitives::structs::{Segment, Point};

enum GraphVal {
    Point,
    Segment,
    None
}

pub struct Graph {
    pub points : Vec<GraphVal>,
    pub segments : Vec<GraphVal>
}

impl Graph {
    pub fn new( points : Vec<GraphVal> , segments: Vec<GraphVal> ) -> Graph {
        return Graph {
            points : points,
            segments : segments
        }
    }

    pub fn draw(&self, ctx : CanvasRenderingContext2d) -> () {
        for seg in self.segments {
            seg.draw(ctx)
        }
        for point in self.points {
            point.draw(ctx)
        }
    }
}