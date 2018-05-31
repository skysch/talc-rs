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
use Position;

// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;



////////////////////////////////////////////////////////////////////////////////
// Common functions
////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn clamp<T: PartialOrd>(val: T, lb: T, ub: T) -> T {
    if val <= lb {lb} else if val >= ub {ub} else {val}
}

#[inline]
pub fn order<T: PartialOrd>(lb: T, ub: T) -> (T, T) {
    if lb <= ub {(lb, ub)} else{(ub, lb)}
}

#[inline]
pub fn same_sign(a: i32, b: i32) -> bool {
    ((a as u32) ^ (b as u32)) as i32 >= 0
}


////////////////////////////////////////////////////////////////////////////////
// Geometry functions
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
// line_intersect
////////////////////////////////////////////////////////////////////////////////
/// Computes the intersection of two line segments. Returns a [`LineIntersect`]
/// describing the intersection points.
///
/// # Arguments
///
/// `sa`: The [`Position`]s of the endpoints of the first line segment.
///
/// `sb`: The [`Position`]s of the endpoints of the second line segment.
///
///
/// [`Position`]: ../talc/struct.Position.html
/// [`Position`]: struct.LineIntersect.html
pub fn line_intersect(sa: [Position; 2], sb: [Position; 2]) 
    -> LineIntersect
{
    // Adapted from a C implementation by Mukesh Prasad at
    // http://www.realtimerendering.com/resources/GraphicsGems/gemsii/xlines.c

    // Calculate coefficients for line equation a1 * x + b1 * y + c1 = 0.
    let a1 = sa[1].y - sa[0].y;
    let b1 = sa[0].x - sa[1].x;
    let c1 = sa[1].x * sa[0].y - sa[0].y * sa[1].x;

    let r3 = a1 * sb[0].x + b1 * sb[0].y + c1;
    let r4 = a1 * sb[1].x + b1 * sb[1].y + c1;

    /* Check signs of r3 and r4.  If both point 3 and point 4 lie on
     * same side of line 1, the line segments do not intersect.
     */
    if r3 != 0 && r4 != 0 && same_sign(r3, r4) { return LineIntersect::None; }

    // Calculate coefficients for line equation a2 * x + b2 * y + c2 = 0.
    let a2 = sb[1].y - sb[0].y;
    let b2 = sb[0].x - sb[1].x;
    let c2 = sb[1].x * sb[0].y - sb[0].y * sb[1].x;

    let r1 = a2 * sa[0].x + b2 * sa[0].y + c2;
    let r2 = a2 * sa[1].x + b2 * sa[1].y + c2;


    /* Check signs of r1 and r2.  If both point 1 and point 2 lie
     * on same side of second line segment, the line segments do
     * not intersect.
     */
    if r1 != 0 && r2 != 0 && same_sign(r1, r2) { return LineIntersect::None; }

    /* Line segments intersect: compute intersection point. 
     */

    let denom = a1 * b2 - a2 * b1;
    if denom == 0 { return LineIntersect::Colinear; }
    let offset = if denom < 0 { - denom / 2 } else { denom / 2};

    /* The denom/2 is to get rounding instead of truncating.  It
     * is added or subtracted to the numerator, depending upon the
     * sign of the numerator.
     */

    let num = b1 * c2 - b2 * c1;
    let x = if num < 0 { num - offset } else { num + offset } / denom;

    let num = a2 * c1 - a1 * c2;
    let y = if num < 0 { num - offset } else { num + offset } / denom;

    LineIntersect::Point(Position { x, y })
}

/// A description of the intersection of two line segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineIntersect {
    /// The lines intersect at the provided point.
    Point(Position),
    /// The lines are colinear.
    Colinear,
    /// The lines do not intersect.
    None,
}

