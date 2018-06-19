// Copyright 20.018 Skylor R. Schermer.
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
use geometry::extend_segment_to_rect;
use geometry::Point;
use geometry::Rect;
use geometry::intersect_segment_with_segment;
use geometry::intersect_line_with_segment;
use geometry::clip_line_to_rect;
use geometry::clip_segment_to_rect;
use geometry::Intersection;

// Standard library imports.
use std::f64::consts::PI;



////////////////////////////////////////////////////////////////////////////////
// intersect_segment_with_segment
////////////////////////////////////////////////////////////////////////////////
#[test]
fn intersect_segment_with_segment_origin_cross() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(-10.0, -10.0), Point::new(10.0, 10.0)], 
        [Point::new(10.0, -10.0), Point::new(-10.0, 10.0)]),
        Intersection::At(Point::new(0.0, 0.0)));
}

#[test]
fn intersect_segment_with_segment_origin_cross_distant() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(-20.0, -20.0), Point::new(-10.0, -10.0)], 
        [Point::new(20.0, -20.0), Point::new(10.0, -10.0)]),
        Intersection::None);
}

#[test]
fn intersect_segment_with_segment_parallel() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(0.0, 0.0), Point::new(5.0, 5.0)], 
        [Point::new(10.0, 10.0), Point::new(15.0, 15.0)]),
        Intersection::Colinear);
}

#[test]
fn intersect_segment_with_segment_at_endpoint() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(0.0, 0.0), Point::new(5.0, 5.0)], 
        [Point::new(2.0, 3.0), Point::new(5.0, 5.0)]),
        Intersection::At(Point::new(5.0, 5.0)));
}

////////////////////////////////////////////////////////////////////////////////
// intersect_line_with_segment
////////////////////////////////////////////////////////////////////////////////
#[test]
fn intersect_line_with_segment_origin_cross_horizontal() {
    assert_eq!(intersect_line_with_segment(
        Point::new(-10.0, 0.0), 0.0,
        [Point::new(10.0, -10.0), Point::new(-10.0, 10.0)]),
        Intersection::At(Point::new(0.0, 0.0)));
}

#[test]
fn intersect_line_with_segment_origin_cross_vertical() {
    assert_eq!(intersect_line_with_segment(
        Point::new(0.0, -10.0), PI / 2.0,
        [Point::new(10.0, -10.0), Point::new(-10.0, 10.0)]),
        Intersection::At(Point::new(0.0, 0.0)));
}

#[test]
fn intersect_line_with_segment_origin_cross() {
    assert_eq!(intersect_line_with_segment(
        Point::new(-10.0, -10.0), PI / 4.0,
        [Point::new(10.0, -10.0), Point::new(-10.0, 10.0)]),
        Intersection::At(Point::new(0.0, 0.0)));
}

#[test]
fn intersect_line_with_segment_parallel() {
    assert_eq!(intersect_line_with_segment(
        Point::new(-10.0, -10.0), PI / 4.0,
        [Point::new(-5.0, -5.0), Point::new(-15.0, -15.0)]),
        Intersection::Colinear);
}

#[test]
fn intersect_line_with_segment_at_endpoint() {
    assert_eq!(intersect_line_with_segment(
        Point::new(-10.0, -10.0), PI / 4.0,
        [Point::new(2.0, 3.0), Point::new(5.0, 5.0)]),
        Intersection::At(Point::new(5.0, 5.0)));
}

////////////////////////////////////////////////////////////////////////////////
// extend_segment_to_rect
////////////////////////////////////////////////////////////////////////////////
#[test]
fn extend_segment_to_rect_vertical_interior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(10.0, 10.0), Point::new(10.0, 60.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(10.0, 0.0), Point::new(10.0, 100.0)]));
}

#[test]
fn extend_segment_to_rect_vertical_overlap() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(10.0, -10.0), Point::new(10.0, 60.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(10.0, 0.0), Point::new(10.0, 100.0)]));
}

#[test]
fn extend_segment_to_rect_vertical_exterior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(10.0, -10.0), Point::new(10.0, -60.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(10.0, 0.0), Point::new(10.0, 100.0)]));
}

#[test]
fn extend_segment_to_rect_vertical_degenerate_edge() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(0.0, -10.0), Point::new(0.0, -60.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(0.0, 0.0), Point::new(0.0, 100.0)]));
}

#[test]
fn extend_segment_to_rect_horizontal_interior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(10.0, 10.0), Point::new(60.0, 10.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(0.0, 10.0), Point::new(100.0, 10.0)]));
}

#[test]
fn extend_segment_to_rect_horizontal_overlap() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(-10.0, 10.0), Point::new(60.0, 10.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(0.0, 10.0), Point::new(100.0, 10.0)]));
}

#[test]
fn extend_segment_to_rect_horizontal_exterior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(-10.0, 10.0), Point::new(-60.0, 10.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(0.0, 10.0), Point::new(100.0, 10.0)]));
}

#[test]
fn extend_segment_to_rect_horizontal_degenerate_edge() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    let segment = [Point::new(-10.0, 0.0), Point::new(-60.0, 0.0)];

    assert_eq!(extend_segment_to_rect(segment, rect),
         Some([Point::new(0.0, 0.0), Point::new(100.0, 0.0)]));
}


////////////////////////////////////////////////////////////////////////////////
// clip_segment_to_rect
////////////////////////////////////////////////////////////////////////////////
#[test]
fn clip_segment_to_rect_interior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(10.0, 10.0), Point::new(10.0, 20.0)], rect),
        Some([Point::new(10.0, 10.0), Point::new(10.0, 20.0)]));
}
#[test]
fn clip_segment_to_rect_exterior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(-10.0, 10.0), Point::new(110.0, 20.0)], rect),
        Some([Point::new(0.0, 10.833333), Point::new(100.0, 19.166668)]));
}

#[test]
fn clip_segment_to_rect_vertical() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(10.0, -10.0), Point::new(10.0, 20.0)], rect),
        Some([Point::new(10.0, 0.0), Point::new(10.0, 20.0)]));
}

#[test]
fn clip_segment_to_rect_horizontal() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(80.0, 10.0), Point::new(120.0, 10.0)], rect),
        Some([Point::new(80.0, 10.0), Point::new(100.0, 10.0)]));
}

#[test]
fn clip_segment_to_rect_vertical_degenerate_edge() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(0.0, -10.0), Point::new(0.0, 20.0)], rect),
        Some([Point::new(0.0, 0.0), Point::new(0.0, 20.0)]));
}

#[test]
fn clip_segment_to_rect_horizontal_degenerate_edge() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(10.0, 0.0), Point::new(20.0, 0.0)], rect),
        Some([Point::new(10.0, 0.0), Point::new(20.0, 0.0)]));
}

#[test]
fn clip_segment_to_rect_degenerate_edge_point() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(-120.0, -10.0), Point::new(65.0, 0.0)], rect),
        Some([Point::new(65.0, 0.0), Point::new(65.0, 0.0)]));
}


#[test]
fn clip_segment_to_rect_degenerate_corner() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(-120.0, -10.0), Point::new(0.0, 0.0)], rect),
        Some([Point::new(0.0, 0.0), Point::new(0.0, 0.0)]));
}


#[test]
fn clip_segment_to_rect_degenerate_corner_tangent() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(200.0, 0.0), Point::new(0.0, 200.0)], rect),
        Some([Point::new(100.0, 100.0), Point::new(100.0, 100.0)]));
}

#[test]
fn clip_segment_to_rect_degenerate_corner_tangent_reverse() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_segment_to_rect(
        [Point::new(0.0, 200.0), Point::new(200.0, 0.0)], rect),
        Some([Point::new(100.0, 100.0), Point::new(100.0, 100.0)]));
}

////////////////////////////////////////////////////////////////////////////////
// clip_line_to_rect
////////////////////////////////////////////////////////////////////////////////
#[test]
fn clip_line_to_rect_vertical() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_line_to_rect(
        Point::new(10.0, -10.0), PI / 2.0, rect),
        Some([Point::new(10.0, 0.0), Point::new(10.0, 100.0)]));
}

#[test]
fn clip_line_to_rect_horizontal() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_line_to_rect(
        Point::new(-10.0, 10.0), PI, rect),
        Some([Point::new(0.0, 10.0), Point::new(100.0, 10.0)]));
}

#[test]
fn clip_line_to_rect_diagonal() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_line_to_rect(
        Point::new(-10.0, -10.0), PI / 4.0, rect),
        Some([Point::new(0.0, 0.0), Point::new(100.0, 100.0)]));
}

#[test]
fn clip_line_to_rect_exterior() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_line_to_rect(
        Point::new(-10.0, -10.0), PI / 2.0, rect),
        None);
}
#[test]
fn clip_line_to_rect_vertical_degenerate_edge() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_line_to_rect(
        Point::new(0.0, -10.0), PI / 2.0, rect),
        Some([Point::new(0.0, 0.0), Point::new(0.0, 100.0)]));
}

#[test]
fn clip_line_to_rect_horizontal_degenerate_edge() {
    let rect = Rect { left: 0.0, top: 0.0, right: 100.0, bottom: 100.0 };

    assert_eq!(clip_line_to_rect(
        Point::new(-10.0, 0.0), 0.0, rect),
        Some([Point::new(0.0, 0.0), Point::new(100.0, 0.0)]));
}
