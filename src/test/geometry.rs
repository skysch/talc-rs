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
use geometry::Rect;
use geometry::Point;
use geometry::segment_intersect;
use geometry::SegmentIntersect;



////////////////////////////////////////////////////////////////////////////////
// segment_intersect
////////////////////////////////////////////////////////////////////////////////
#[test]
fn origin_cross_segment() {
    assert_eq!(segment_intersect(
        [Point::new(-10, -10), Point::new(10, 10)], 
        [Point::new(10, -10), Point::new(-10, 10)]),
        SegmentIntersect::Point(Point::new(0, 0)));
}

#[test]
fn origin_cross_distant_segment() {
    assert_eq!(segment_intersect(
        [Point::new(-20, -20), Point::new(-10, -10)], 
        [Point::new(20, -20), Point::new(10, -10)]),
        SegmentIntersect::None);
}

#[test]
fn parallel_segment() {
    assert_eq!(segment_intersect(
        [Point::new(0, 0), Point::new(5, 5)], 
        [Point::new(10, 10), Point::new(15, 15)]),
        SegmentIntersect::Colinear);
}


////////////////////////////////////////////////////////////////////////////////
// Rect
////////////////////////////////////////////////////////////////////////////////

#[test]
fn rect_spanning_vertical_interior() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(10, 10), Point::new(10, 60)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(10, 0), Point::new(10, 100)]));
}

#[test]
fn rect_spanning_vertical_overlap() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(10, -10), Point::new(10, 60)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(10, 0), Point::new(10, 100)]));
}

#[test]
fn rect_spanning_vertical_exterior() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(10, -10), Point::new(10, -60)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(10, 0), Point::new(10, 100)]));
}

#[test]
fn rect_spanning_vertical_degenerate_edge() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(0, -10), Point::new(0, -60)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(0, 0), Point::new(0, 100)]));
}

#[test]
fn rect_spanning_horizontal_interior() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(10, 10), Point::new(60, 10)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(0, 10), Point::new(100, 10)]));
}

#[test]
fn rect_spanning_horizontal_overlap() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(-10, 10), Point::new(60, 10)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(0, 10), Point::new(100, 10)]));
}

#[test]
fn rect_spanning_horizontal_exterior() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(-10, 10), Point::new(-60, 10)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(0, 10), Point::new(100, 10)]));
}

#[test]
fn rect_spanning_horizontal_degenerate_edge() {
    let rect = Rect { left: 0, top: 0, right: 100, bottom: 100 };

    let segment = [Point::new(-10, 0), Point::new(-60, 0)];

    assert_eq!(rect.spanning_segment(segment),
         Some([Point::new(0, 0), Point::new(100, 0)]));
}
