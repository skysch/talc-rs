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
use geometry::Position;
use utilities::clamp;
use geometry::line_intersect::line_intersect;

////////////////////////////////////////////////////////////////////////////////
// line_rect_intersect
////////////////////////////////////////////////////////////////////////////////

pub fn line_rect_intersect(
    segment: [Position; 2],
    rect: [Position; 2])
    -> Option<[Position; 2]>
{
    use geometry::line_intersect::LineIntersect::*;

    // Get edge segments.
    let h0 = [rect[0], Position {x: rect[1].x, y: rect[0].y}];
    let h1 = [rect[1], Position {x: rect[0].x, y: rect[1].y}];
    let v0 = [rect[0], Position {x: rect[0].x, y: rect[1].y}];
    let v1 = [rect[1], Position {x: rect[1].x, y: rect[0].y}];

    // Intersect edges with the segment.
    let mut h0i = line_intersect(segment, h0);
    let mut h1i = line_intersect(segment, h1);
    let mut v0i = line_intersect(segment, v0);
    let mut v1i = line_intersect(segment, v1);

    // If the intersection is outside the rect, invalidate it.
    if let Point(p) = h0i { if !p.contained_in(rect) { h0i = None } };
    if let Point(p) = h1i { if !p.contained_in(rect) { h1i = None } };
    if let Point(p) = v0i { if !p.contained_in(rect) { v0i = None } };
    if let Point(p) = v1i { if !p.contained_in(rect) { v1i = None } };

    match (h0i, h1i, v0i, v1i) {
        // Line follows edge of the rect. (Must precede other sections, because 
        // colinear on one edge means two intersection points elsewhere.)
        (Colinear,  _,         _,         _)         => Some([
            segment[0].clamp_x(h0[0].x, h0[1].x), 
            segment[1].clamp_x(h0[0].x, h0[1].x), 
        ]),
        (_,         Colinear,  _,         _)         => Some([
            segment[0].clamp_x(h1[0].x, h1[1].x), 
            segment[1].clamp_x(h1[0].x, h1[1].x), 
        ]),
        (_,         _,         Colinear,  _)         => Some([
            segment[0].clamp_y(v0[0].y, v0[1].y), 
            segment[1].clamp_y(v0[0].y, v0[1].y), 
        ]),
        (_,         _,         _,         Colinear)  => Some([
            segment[0].clamp_y(v1[0].y, v1[1].y), 
            segment[1].clamp_y(v1[0].y, v1[1].y), 
        ]),

        // Line intersects two edges of the rect. (Must precede single edge 
        // intersections, which are struct subsets of this.)
        (Point(p1), Point(p2), _,         _)         |
        (Point(p1), _,         Point(p2), _)         |
        (Point(p1), _,         _,         Point(p2)) |
        (_,         Point(p1), Point(p2), _)         |
        (_,         Point(p1), _,         Point(p2)) |
        (_,         _,         Point(p1), Point(p2)) => Some([p1, p2]),


        // Line intersects one edge of the rect.
        (Point(p),  _,         _,         _)         |
        (_,         Point(p),  _,         _)         |
        (_,         _,         Point(p),  _)         |
        (_,         _,         _,         Point(p))  => {
            if segment[0].contained_in(rect) {
                Some([segment[0], p])
            } else {
                debug_assert!(segment[1].contained_in(rect));
                Some([segment[1], p])
            }
        },

        // Line intersects no edges. Must be entirely inside or outside.
        (None,      None,      None,      None)      => {
            if segment[0].contained_in(rect) {
                debug_assert!(segment[1].contained_in(rect));
                Some(segment)
            } else {
                Option::None
            }
        },
    }
}