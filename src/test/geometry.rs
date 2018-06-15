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
use geometry::Intersection;



////////////////////////////////////////////////////////////////////////////////
// intersect_segment_with_segment
////////////////////////////////////////////////////////////////////////////////
#[test]
fn origin_cross_segment() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(-10.0, -10.0), Point::new(10.0, 10.0)], 
        [Point::new(10.0, -10.0), Point::new(-10.0, 10.0)]),
        Intersection::At(Point::new(0.0, 0.0)));
}

#[test]
fn origin_cross_distant_segment() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(-20.0, -20.0), Point::new(-10.0, -10.0)], 
        [Point::new(20.0, -20.0), Point::new(10.0, -10.0)]),
        Intersection::None);
}

#[test]
fn parallel_segment() {
    assert_eq!(intersect_segment_with_segment(
        [Point::new(0.0, 0.0), Point::new(5.0, 5.0)], 
        [Point::new(10.0, 10.0), Point::new(15.0, 15.0)]),
        Intersection::Colinear);
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
