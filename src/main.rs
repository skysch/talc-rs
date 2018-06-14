



extern crate talc;


use talc::*;
// use talc::geometry::Rect;
use talc::geometry::convert_line_to_segment;


pub fn main() {

	use std::f64::consts::PI;

	let step = 2.0 * PI / 16.0;
	for i in 0..16 {

		let seg = convert_line_to_segment(Point::new(0.0, 0.0),  step * i as f64);
		println!("{:?}", seg);

	}

}