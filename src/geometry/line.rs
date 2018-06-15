// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Common line algorithms and primitives.
//!
////////////////////////////////////////////////////////////////////////////////
//
// Line Algorithms
// -------------------
// 
// segment_intersect          Solve line system ax+bx+c = 0
// 
// convert_line_to_segment    Solve line equation mx+b-y = 0
// convert_ray_to_segment     Solve line equation mx+b-y = 0
// 
// extend_segment_to_rect     Segment intersect each side of rect
//
// clip_segment_to_rect       Liang-Barksy algorithm
// clip_line_to_rect          Convert line to segment, then clip segment to rect
// clip_ray_to_rect           Convert ray to segment, then clip segment to rect
// clip_segment_to_poly       Cyrus-Beck algorithm
// 
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use geometry::angle::angle_classify;
use geometry::angle::angle_shift;
use geometry::angle::AngleType;
use geometry::Point;
use geometry::Rect;
use std::f32;
use utilities::same_sign;


////////////////////////////////////////////////////////////////////////////////
// Intersection
////////////////////////////////////////////////////////////////////////////////
/// A description of the intersection of lines or segments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intersection {
    /// The lines or segments intersect at the provided point.
    At(Point),
    /// The lines or segments are colinear.
    Colinear,
    /// The lines or segments do not intersect.
    None,
}

////////////////////////////////////////////////////////////////////////////////
// intersect_segment_with_segment
////////////////////////////////////////////////////////////////////////////////
/// Computes the intersection of two line segments. Returns an [`Intersection`]
/// describing the intersection point.
///
/// # Arguments
///
/// `epa`: The [`Point`]s of the endpoints of the first line segment.
///
/// `epb`: The [`Point`]s of the endpoints of the second line segment.
///
/// [`Point`]: ../talc/struct.Point.html
/// [`Point`]: struct.Intersection.html
pub fn intersect_segment_with_segment(epa: [Point; 2], epb: [Point; 2]) 
    -> Intersection
{
    // Adapted from C implementation by Mukesh Prasad at
    // http://www.realtimerendering.com/resources/GraphicsGems/gemsii/xlines.c

    // Calculate coefficients for line equation a1 * x + b1 * y + c1 = 0.
    let a1 = epa[1].y - epa[0].y;
    let b1 = epa[0].x - epa[1].x;
    let c1 = epa[1].x * epa[0].y - epa[0].x * epa[1].y;

    // Solve equation for endpoints of other segment.
    let ab_r0 = a1 * epb[0].x + b1 * epb[0].y + c1;
    let ab_r1 = a1 * epb[1].x + b1 * epb[1].y + c1;

    // Zeros mean the endpoints lie on the line. Otherwise, if they have the
    // same sign, they are on the same side of the line and can't intersect it.
    if ab_r0 != 0.0 && ab_r1 != 0.0 && same_sign(ab_r0, ab_r1) {
        return Intersection::None;
    }

    // Calculate coefficients for line equation a2 * x + b2 * y + c2 = 0.
    let a2 = epb[1].y - epb[0].y;
    let b2 = epb[0].x - epb[1].x;
    let c2 = epb[1].x * epb[0].y - epb[0].x * epb[1].y;

    // Solve equation for endpoints of other segment.
    let ba_r0 = a2 * epa[0].x + b2 * epa[0].y + c2;
    let ba_r1 = a2 * epa[1].x + b2 * epa[1].y + c2;

    // Zeros mean the endpoints lie on the line. Otherwise, if they have the
    // same sign, they are on the same side of the line and can't intersect it.
    if ba_r0 != 0.0 && ba_r1 != 0.0 && same_sign(ba_r0, ba_r1) {
        return Intersection::None;
    }

    // Line segments intersect. Compute intersection point. 
    let denom = a1 * b2 - a2 * b1;
    if denom == 0.0 { return Intersection::Colinear; }

    // If we wanted to return integers with round-to-nearest behavior, we would
    // divide the denominator by 2 here. Instead, we just return the actual
    // interection point.
    let offset = 0.0;
    // let offset = if denom < 0.0 { -denom / 2.0 } else { denom / 2.0 };

    let dx = b1 * c2 - b2 * c1;
    let x = if dx < 0.0 { dx - offset } else { dx + offset } / denom;

    let dy = a2 * c1 - a1 * c2;
    let y = if dy < 0.0 { dy - offset } else { dy + offset } / denom;

    Intersection::At(Point { x, y })
}



////////////////////////////////////////////////////////////////////////////////
// intersect_line_with_segment
////////////////////////////////////////////////////////////////////////////////
/// Computes the intersection of two line segments. Returns a
/// [`Intersection`] describing the intersection points.
///
/// # Arguments
///
/// `epa`: The [`Point`]s of the endpoints of the first line segment.
///
/// `epb`: The [`Point`]s of the endpoints of the second line segment.
///
/// [`Point`]: ../talc/struct.Point.html
/// [`Point`]: struct.Intersection.html
pub fn intersect_line_with_segment(pt: Point, angle: f64, segment: [Point; 2])
    -> Intersection
{
    let theta = angle_shift(angle, 0.0);
   
    Intersection::None
}

////////////////////////////////////////////////////////////////////////////////
// intersect_segment_with_rect
////////////////////////////////////////////////////////////////////////////////
/// Returns the largest segment colinear with the given line that lies
/// entirely within the `Rect`.
pub fn intersect_segment_with_rect(segment: [Point; 2], rect: Rect)
    -> Option<[Point; 2]>
{
    /// This is an implementation of the Liang-Barsky algorithm.
    
    None

}

////////////////////////////////////////////////////////////////////////////////
// intersect_line_with_rect
////////////////////////////////////////////////////////////////////////////////
/// Returns the largest segment colinear with the given line that lies
/// entirely within the `Rect`.
pub fn intersect_line_with_rect(pt: Point, angle: f64, rect: Rect)
    -> Option<[Point; 2]>
{
    /// This is an implementation of the Liang-Barsky algorithm.
    
    None

}

////////////////////////////////////////////////////////////////////////////////
// convert_line_to_segment
////////////////////////////////////////////////////////////////////////////////
pub fn convert_line_to_segment(pt: Point, angle: f64) -> [Point; 2] {
    match angle_classify(angle) {
        AngleType::Invalid       => panic!("invalid line angle"),
        AngleType::Horizontal    => [
            Point { x: f32::MIN, y: pt.y },
            Point { x: f32::MAX, y: pt.y },
        ],
        AngleType::Vertical      => [
            Point { x: pt.x, y: f32::MIN },
            Point { x: pt.x, y: f32::MAX },
        ],
        AngleType::Normal(theta) => {
            let (min, max) = (f32::MIN as f64, f32::MAX as f64);
            let (sin_theta, cos_theta) = theta.sin_cos();

            // Neither sin or cos can be 1 or 0.0 here.
            let (x, y) = (pt.x as f64, pt.y as f64);
            let m = sin_theta / cos_theta; 
            let b = y - m * x;

            let y_xmin = Some((min - b) / m).filter(|&v| v >= min && v <= max);
            let y_xmax = Some((max - b) / m).filter(|&v| v >= min && v <= max);
            let x_ymin = Some(m * min + b).filter(|&v| v >= min && v <= max);
            let x_ymax = Some(m * max + b).filter(|&v| v >= min && v <= max);


            // match (y_xmin, y_xmax, x_ymin, x_ymax) {
            //     (Some(y_xmin), Some(y_xmax),            _,           _) => [
            //         Point { x: min, y: y_xmin },
            //         Point { x: max, y: y_xmax },
            //     ],
            //     (Some(y_xmin),            _, Some(x_ymin),            _) => 
            //     (Some(y_xmin),            _,            _, Some(x_ymax)) => 

            //     (Some(y_xmin), Some(y_xmax), Some(x_ymin), Some(x_ymax)) => 
            // }

            [pt, pt]
        },
    }    
}

////////////////////////////////////////////////////////////////////////////////
// convert_ray_to_segment
////////////////////////////////////////////////////////////////////////////////
pub fn convert_ray_to_segment(pt: Point, angle: f64) -> [Point; 2] {
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////
// extend_segment_to_rect
////////////////////////////////////////////////////////////////////////////////
pub fn extend_segment_to_rect(segment: [Point; 2], rect: Rect) -> Option<[Point; 2]>
{
    // Terse segment constructor.
    #[inline]
    fn s(x1: f32, y1: f32, x2: f32, y2: f32) -> Option<[Point; 2]> {
        Some([Point { x: x1, y: y1 },  Point { x: x2, y: y2 }])
    }

    let [ea, eb] = segment;
    if ea == eb {
        panic!("segment endpoints do not determine a unique line");
    }

    // Calculate coefficients for line equation a*x + b*y + c = 0.
    let a = eb.y - ea.y;
    let b = ea.x - eb.x;
    let c = eb.x * ea.y - ea.x * eb.y;

    if a == 0.0 && rect.contains_y(eb.y) {
        // Horizontal line.
        s(rect.left, eb.y, rect.right, eb.y)

    } else if b == 0.0 && rect.contains_x(eb.x) {
        // Vertical line.
        s(eb.x, rect.top, eb.x, rect.bottom)

    } else if a == 0.0 || b == 0.0 {
        // Vertical or horizontal line out of rect.
        None

    } else {
        // Find edge intesections.
        let tx = (-b * rect.top - c) / a;
        let bx = (-b * rect.bottom - c) / a;
        let ly = (-a * rect.left - c) / b;
        let ry = (-a * rect.right - c) / b;

        // Edge intersections in rect.
        let mut txi = rect.contains_x(tx);
        let mut bxi = rect.contains_x(bx);
        let mut lyi = rect.contains_y(ly);
        let mut ryi = rect.contains_y(ry);

        // Normalize corner behavior.
        if tx == ly { lyi = false };
        if tx == ry { ryi = false };
        if bx == ly { lyi = false };
        if bx == ry { ryi = false };

        // Build spanning segment.
        match (txi, bxi, lyi, ryi) {
            (true, true, _,    _   ) => s(tx, rect.top, bx, rect.bottom),
            (true, _,    true, _   ) => s(tx, rect.top, rect.left, ly),
            (true, _,    _,    true) => s(tx, rect.top, rect.right, ry),
            (_,    true, true, _   ) => s(bx, rect.bottom, rect.left, ly),
            (_,    true, _,    true) => s(bx, rect.bottom, rect.right, ry),
            (_,    _,    true, true) => s(rect.left, ly, rect.right, ry),

            // Degenerate segment in upper corners.
            (true, _,    _,    _   ) => if tx == ly {
                    s(rect.left, rect.top, rect.left, rect.top)
                } else {
                    s(rect.right, rect.top, rect.right, rect.top)
                },

            // Degenerate segment in lower corners.
            (_,    true, _,    _   ) => if bx == ly {
                    s(rect.left, rect.bottom, rect.left, rect.bottom)
                } else {
                    s(rect.right, rect.bottom, rect.right, rect.bottom)
                },

            // Non-intesecting line.
            _                        => None,
        }
    }
}





////////////////////////////////////////////////////////////////////////////////
// clip_segment_to_rect
////////////////////////////////////////////////////////////////////////////////

pub fn clip_segment_to_rect(
    segment: [Point; 2],
    rect: [Point; 2])
    -> Option<[Point; 2]>
{
    use geometry::line::Intersection::*;

    // Get edge segments.
    let h0 = [rect[0], Point {x: rect[1].x, y: rect[0].y}];
    let h1 = [rect[1], Point {x: rect[0].x, y: rect[1].y}];
    let v0 = [rect[0], Point {x: rect[0].x, y: rect[1].y}];
    let v1 = [rect[1], Point {x: rect[1].x, y: rect[0].y}];

    // Intersect edges with the segment.
    let mut h0i = intersect_segment_with_segment(segment, h0);
    let mut h1i = intersect_segment_with_segment(segment, h1);
    let mut v0i = intersect_segment_with_segment(segment, v0);
    let mut v1i = intersect_segment_with_segment(segment, v1);

    // If the intersection is outside the rect, invalidate it.
    if let At(p) = h0i { if !p.contained_in(rect) { h0i = None } };
    if let At(p) = h1i { if !p.contained_in(rect) { h1i = None } };
    if let At(p) = v0i { if !p.contained_in(rect) { v0i = None } };
    if let At(p) = v1i { if !p.contained_in(rect) { v1i = None } };

    match (h0i, h1i, v0i, v1i) {
        // Line follows edge of the rect. (Must precede other sections, because 
        // colinear on one edge means two intersection points elsewhere.)
        (Colinear,  _,         _,         _)         => {
            if segment[0].contained_in(rect) || segment[1].contained_in(rect) {
                Some([
                    segment[0].clamped_x(h0[0].x, h0[1].x), 
                    segment[1].clamped_x(h0[0].x, h0[1].x), 
                ])
            } else {
                Option::None
            }
        },

        (_,         Colinear,  _,         _)         => {
            if segment[0].contained_in(rect) || segment[1].contained_in(rect) {
                Some([
                    segment[0].clamped_x(h1[0].x, h1[1].x), 
                    segment[1].clamped_x(h1[0].x, h1[1].x), 
                ])
            } else {
                Option::None
            }
        },

        (_,         _,         Colinear,  _)         => {
            if segment[0].contained_in(rect) || segment[1].contained_in(rect) {
                Some([
                    segment[0].clamped_y(v0[0].y, v0[1].y), 
                    segment[1].clamped_y(v0[0].y, v0[1].y), 
                ])
            } else {
                Option::None
            }
        },

        (_,         _,         _,         Colinear)  => {
            if segment[0].contained_in(rect) || segment[1].contained_in(rect) {
                Some([
                    segment[0].clamped_y(v1[0].y, v1[1].y), 
                    segment[1].clamped_y(v1[0].y, v1[1].y), 
                ])
            } else {
                Option::None
            }
        },

        // Line intersects two edges of the rect. (Must precede single edge 
        // intersections, which are struct subsets of this.)
        (At(p1), At(p2), _,      _)      |
        (At(p1), _,      At(p2), _)      |
        (At(p1), _,      _,      At(p2)) |
        (_,      At(p1), At(p2), _)      |
        (_,      At(p1), _,      At(p2)) |
        (_,      _,      At(p1), At(p2)) => Some([p1, p2]),


        // Line intersects one edge of the rect.
        (At(p),  _,         _,         _)      |
        (_,         At(p),  _,         _)      |
        (_,         _,         At(p),  _)      |
        (_,         _,         _,      At(p))  => {
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

////////////////////////////////////////////////////////////////////////////////
// clip_line_to_rect
////////////////////////////////////////////////////////////////////////////////
pub fn clip_line_to_rect(
    pt: Point,
    angle: f64,
    rect: [Point; 2])
    -> Option<[Point; 2]>
{
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////
// clip_ray_to_rect
////////////////////////////////////////////////////////////////////////////////
pub fn clip_ray_to_rect(
    pt: Point,
    angle: f64,
    rect: [Point; 2])
    -> Option<[Point; 2]>
{
    unimplemented!()
}


////////////////////////////////////////////////////////////////////////////////
// clip_segment_to_poly
////////////////////////////////////////////////////////////////////////////////
pub fn clip_segment_to_poly(
    pt: Point,
    angle: f64,
    rect: [Point; 2])
    -> Option<[Point; 2]>
{
    // Cyrus-Beck algorithm.
    unimplemented!()
}