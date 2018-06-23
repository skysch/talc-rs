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

// Local imports.
use geometry::angle::angle_classify;
use geometry::angle::AngleType;
use geometry::Point;
use geometry::Rect;
use utilities::clipped;
use utilities::ordered;
use utilities::same_sign;

// Standard library imports.
use std::f32;

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


enum EdgeIntersection {
    Colinear([Point; 2]),
    At(Point),
}

struct RectEdgeIter {
    rect: Rect,
    edge_num: u8,
}
impl RectEdgeIter {
    pub fn new(rect: Rect) -> Self {
        RectEdgeIter { rect, edge_num: 0 }
    }
}

impl Iterator for RectEdgeIter {
    type Item = [Point; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.edge_num >= 4 {
            None
        } else {
            self.edge_num += 1;
            match self.edge_num {
                1 => Some([ // Top
                    Point { x: self.rect.left, y: self.rect.top },
                    Point { x: self.rect.right, y: self.rect.top },
                ]),
                2 => Some([ // Left
                    Point { x: self.rect.left, y: self.rect.top },
                    Point { x: self.rect.left, y: self.rect.bottom },
                ]),
                3 => Some([ // Right
                    Point { x: self.rect.right, y: self.rect.top },
                    Point { x: self.rect.right, y: self.rect.bottom },
                ]),
                4 => Some([ // Bottom
                    Point { x: self.rect.left, y: self.rect.bottom },
                    Point { x: self.rect.right, y: self.rect.bottom },
                ]),
                _ => unreachable!(),
            }
        }
    }
}


struct RectEdgeIntersectIter {
    angle: f64,
    pt: Point,
    edges: RectEdgeIter,
}

impl RectEdgeIntersectIter {
    pub fn new(pt: Point, angle: f64, rect: Rect) -> Self {
        RectEdgeIntersectIter {  angle, pt, edges: RectEdgeIter::new(rect) }
    }
}

impl Iterator for RectEdgeIntersectIter {
    type Item = EdgeIntersection;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(edge) = self.edges.next() {
            match intersect_line_with_segment(self.pt, self.angle, edge) {
                Intersection::Colinear => {
                    return Some(EdgeIntersection::Colinear(edge));
                },
                Intersection::At(pt) => {
                    return Some(EdgeIntersection::At(pt));
                },
                Intersection::None => {
                    /* Do nothing. */
                },
            }
        }
        None
    }
}

////////////////////////////////////////////////////////////////////////////////
// intersect_segment_with_segment
////////////////////////////////////////////////////////////////////////////////
/// Computes the intersection of two line segments. Returns an [`Intersection`]
/// describing the intersection [`Point`].
///
/// # Arguments
///
/// `epa`: The endpoints of the first line segment.
///
/// `epb`: The endpoints of the second line segment.
///
/// [`Point`]: ../talc/struct.Point.html
/// [`Intersection`]: struct.Intersection.html
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
    if ab_r0 == 0.0 && ab_r1 == 0.0 {
        return Intersection::Colinear;
    } else if same_sign(ab_r0, ab_r1) {
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
    // If we wanted to return integers with round-to-nearest behavior, we would
    // divide the denominator by 2 here. Instead, we just return the logical
    // interection point.
    let denom = a1 * b2 - a2 * b1;
    let x = (b1 * c2 - b2 * c1) / denom;
    let y = (a2 * c1 - a1 * c2) / denom;

    Intersection::At(Point { x, y })
}



////////////////////////////////////////////////////////////////////////////////
// intersect_line_with_segment
////////////////////////////////////////////////////////////////////////////////
/// Computes the intersection of a line with a line segment. Returns an
/// [`Intersection`] describing the intersection [`Point`].
///
/// # Arguments
///
/// `pt`: A point on the line.
///
/// `angle`: The line's angle with respect to the positive x-axis.
///
/// `segment`: The endpoints of the line segment.
///
/// [`Point`]: ../talc/struct.Point.html
/// [`Intersection`]: struct.Intersection.html
pub fn intersect_line_with_segment(pt: Point, angle: f64, segment: [Point; 2])
    -> Intersection
{
    if segment[0] == segment[1] { panic!("invalid segment"); }

    // Calculate coefficients for line equation ax + by + c = 0.
    let a = segment[1].y - segment[0].y;
    let b = segment[0].x - segment[1].x;
    let c = segment[1].x * segment[0].y - segment[0].x * segment[1].y;

    match angle_classify(angle) {
        AngleType::Invalid       => panic!("invalid line angle"),
        AngleType::Horizontal    => {
            let (y0, y1) = ordered(segment[0].y, segment[1].y);

            if pt.y >= y0 && pt.y <= y1 && a != 0.0 {
                let x = (-b * pt.y - c) / a;
                Intersection::At(Point { x, y: pt.y })

            } else if pt.y == y0 && pt.y == y1 && a == 0.0 {
                Intersection::Colinear

            } else {
                Intersection::None
            }
        },
        AngleType::Vertical      => {
            let (x0, x1) = ordered(segment[0].x, segment[1].x);

            if pt.x >= x0 && pt.x <= x1 && b != 0.0 {
                let y = (-a * pt.x - c) / b;
                Intersection::At(Point { x: pt.x, y })

            } else if pt.x == x0 && pt.x == x1 && b == 0.0 {
                Intersection::Colinear

            } else {
                Intersection::None
            }
        },
        AngleType::Normal(theta) => {
            // Compute line equation mx + b - y = 0
            let (sin_theta, cos_theta) = theta.sin_cos();
            // Neither sin or cos can be 1 or 0.0 here.
            let m = sin_theta as f32 / cos_theta as f32;
            let y_0 = pt.y - m * pt.x;

            // Solve equation for endpoints of other segment.
            let r0 = m * segment[0].x + y_0 - segment[0].y;
            let r1 = m * segment[1].x + y_0 - segment[1].y;

            // Zeros mean the endpoints lie on the line. Otherwise, if they have
            // the same sign, they are on the same side of the line and can't
            // intersect it.
            if r0 == 0.0 && r1 == 0.0 {
                return Intersection::Colinear;
            } else if same_sign(r0, r1) {
                return Intersection::None;
            }

            // Solve system of equations for intersection point.
            let x = (- b * y_0 - c) / (a + m * b);
            let y = m * x + y_0;

            Intersection::At(Point { x, y })
        }
    }
}



////////////////////////////////////////////////////////////////////////////////
// extend_segment_to_rect
////////////////////////////////////////////////////////////////////////////////
/// Extends a line segment's endpoints to the boundaries of the [`Rect`].
/// Returns `None` if the line colinear with the line segment fails to intersect
/// the `Rect`.
///
/// # Arguments
///
/// `segment`: The endpoints of the line segment.
///
/// `rect`: The boundary `Rect`.
///
/// # Panics
///
/// Panics if the segment's endpoints are equal.
///
/// [`Rect`]: ../talc/struct.Rect.html
pub fn extend_segment_to_rect(segment: [Point; 2], rect: Rect)
    -> Option<[Point; 2]>
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
        let txi = rect.contains_x(tx);
        let bxi = rect.contains_x(bx);
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
/// Clips a line segment to the boundaries of the given `Rect`. Returns `None`
/// if the line segment lies entirely outside of the `Rect`.
///
/// # Arguments
///
/// `segment`: The endpoints of the line segment.
///
/// `rect`: The boundary `Rect`.
///
/// [`Rect`]: ../talc/struct.Rect.html
pub fn clip_segment_to_rect(segment: [Point; 2], rect: Rect)
    -> Option<[Point; 2]>
{
    let [Point {x: xa, y: ya}, Point {x: xb, y: yb}] = segment;

    // Liang-Barky line clipping algorithm. Based on the parametric line segment
    // equations for t in [0.0, 1.0]:
    //     x = xa + dx * t;
    //     y = ya + dy * t;
    let dx = xb - xa;
    let dy = yb - ya;
    
    // Handle aligned clipping specially to account for lines colinear with
    // edges.
    if dx == 0.0 {
        return if xa >= rect.left && xa < rect.right {
            clipped((ya, yb), rect.top, rect.bottom)
                .map(|(t, b)| [Point {x: xa, y: t}, Point {x: xa, y: b}])
        } else {
            None
        }
    }
    if dy == 0.0 {
        return if ya >= rect.top && ya < rect.bottom {
            clipped((xa, xb), rect.left, rect.right)
                .map(|(l, r)| [Point {x: l, y: ya}, Point {x: r, y: ya}])
        } else {
            None
        }
    }

    // The initial t values parameterizing the full segment. We want to narrow
    // this to the intersections with each edge.
    let mut t = (0.0, 1.0); 

    // Constraint on top edge.
    let t_edge = (rect.top - ya) / dy;
    if t_edge >= t.0 && t_edge <= t.1 {
        if -dy <= 0.0 { t.0 = t_edge; } else { t.1 = t_edge; }
    }

    // Constraint on left edge.
    let t_edge = (rect.left - xa) / dx;
    if t_edge >= t.0 && t_edge <= t.1 {
        if -dx <= 0.0 { t.0 = t_edge; } else { t.1 = t_edge; }
    }

    // Constraint on right edge.
    let t_edge = (rect.right - xa) / dx;
    if t_edge >= t.0 && t_edge <= t.1 {
        if dx <= 0.0 { t.0 = t_edge; } else { t.1 = t_edge; }
    }

    // Constraint on bottom edge.
    let t_edge = (rect.bottom - ya) / dy;
    if t_edge >= t.0 && t_edge <= t.1 {
        if dy <= 0.0 { t.0 = t_edge; } else { t.1 = t_edge; }
    }

    // If the t values have flipped their compare, then the segment is outside
    // the rect. Otherwise, we can use them to identify the bounds of the
    // clipped segment.
    if t.0 <= t.1 {
        Some([
            Point { x: xa + dx * t.0, y: ya + dy * t.0 },
            Point { x: xa + dx * t.1, y: ya + dy * t.1 },
        ])
    } else {
        None
    }
}


////////////////////////////////////////////////////////////////////////////////
// clip_line_to_rect
////////////////////////////////////////////////////////////////////////////////
/// Clips a line to the boundaries of the given `Rect`. Returns `None` if the
/// line lies entirely outside of the `Rect`.
///
/// # Arguments
///
/// `pt`: A [`Point`] on the line.
///
/// `angle`: The angle of the line with respect to the positive x-axis.
///
/// `rect`: The boundary `Rect`.
///
/// [`Point`]: ../talc/struct.Point.html
/// [`Rect`]: ../talc/struct.Rect.html
pub fn clip_line_to_rect(pt: Point, angle: f64, rect: Rect)
    -> Option<[Point; 2]>
{
    
    let mut stepper = RectEdgeIntersectIter::new(pt, angle, rect);

    // Iterate over each edge, attempting to intersect with it. We can return
    // early if we hit a colinear intersection or if we hit two different
    // intersection points. If we hit two points that are the same, we might
    // have a corner tangent line or a crossing line. We know it is a crossing
    // line if we hit another point later on. If it is a tangent line, we will
    // find no other points and we can reconstruct it at the end. (Hitting only
    // one point should be impossible because edges coincide at their
    // endpoints.)

    let mut save = None;
    while let Some(next) = stepper.next() {
        match next {
            EdgeIntersection::Colinear(edge) => {
                return Some(edge);
            },

            EdgeIntersection::At(pt)         => {
                if save.is_none() {
                    save = Some(pt);
                } else if save.as_ref() != Some(&pt) {
                    return Some([save.unwrap(), pt]);
                }
            },
        }
    }

    save.map(|pt| [pt, pt])

}
