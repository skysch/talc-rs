



extern crate talc;


// use std::f64::consts::PI;
use talc::geometry::clip_segment_to_rect;
use talc::Point;
use talc::Rect;


pub fn main() {
	let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

	println!("{:?}", clip_segment_to_rect(
        [Point::new(-10.0, 10.0), Point::new(110.0, 20.0)], rect));
}