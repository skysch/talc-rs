// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use Canvas;
use Position;


use utilities::LineIntersect;
use utilities::line_intersect;
use utilities::same_sign;

#[test]
fn same_signs() {
	assert!(same_sign(-1, -1));
	assert!(same_sign(1, 1));
	assert!(!same_sign(-1, 1));
	assert!(!same_sign(1, -1));
}

////////////////////////////////////////////////////////////////////////////////
// line_intersect
////////////////////////////////////////////////////////////////////////////////
#[test]
fn origin_cross() {

    // assert_eq!(line_intersect(
    //     [Position::new(-10, -10), Position::new(10, 10)], 
    //     [Position::new(10, -10), Position::new(-10, 10)]),
    //     LineIntersect::Point(Position::new(0, 0)));

    assert_eq!(line_intersect(
        [Position::new(0, 0), Position::new(10, 0)], 
        [Position::new(5, 5), Position::new(5, -10)]),
        LineIntersect::Point(Position::new(5, 0)));
}
