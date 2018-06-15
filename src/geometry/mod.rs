// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Common geometry algorithms and primitives.
//!
////////////////////////////////////////////////////////////////////////////////
//
// Geometry Algorithms
// -------------------
//
// clip_rect_outline_to_rect  Clip each segment to rect
// clip_poly_outline_to_rect  Clip each segment to rect
// clip_rect_to_rect          Simple coordinate clip
// clip_poly_to_rect          Sutherland–Hodgman algorithm
// clip_poly_to_poly          Vatti clipping algorithm
// 
////////////////////////////////////////////////////////////////////////////////

// Internal modules.
mod angle;
mod line;

// Local imports.
use utilities::clamped;
use utilities::ordered;

// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::f32;

// Exports.
pub use self::line::clip_line_to_rect;
pub use self::line::clip_ray_to_rect;
pub use self::line::clip_segment_to_poly;
pub use self::line::clip_segment_to_rect;
pub use self::line::convert_line_to_segment;
pub use self::line::convert_ray_to_segment;
pub use self::line::extend_segment_to_rect;
pub use self::line::intersect_segment_with_segment;
pub use self::line::intersect_line_with_segment;
pub use self::line::intersect_segment_with_rect;
pub use self::line::intersect_line_with_rect;
pub use self::line::Intersection;

////////////////////////////////////////////////////////////////////////////////
// Point
////////////////////////////////////////////////////////////////////////////////
/// A point in a 2-dimensional integer plane.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


impl Point {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn one() -> Self {
        Point { x: 1.0, y: 1.0 }
    }

    #[inline]
    pub fn contained_in(self, rect: [Point; 2]) -> bool {
        let (l, r) = ordered(rect[0].x, rect[1].x);
        let (t, b) = ordered(rect[0].y, rect[1].y);

        self.x >= l && self.x < r && self.y >= t && self.y < b
    }
    
    #[inline]
    pub fn x_ordered(pair: [Point; 2]) -> [Point; 2] {
        if pair[0].x > pair[1].x {
            [pair[1], pair[0]]
        } else {
            pair
        }
    }

    #[inline]
    pub fn y_ordered(pair: [Point; 2]) -> [Point; 2] {
        if pair[0].y > pair[1].y {
            [pair[1], pair[0]]
        } else {
            pair
        }
    }

    #[inline]
    pub fn clamped_x(self, lower_bound: f32, upper_bound: f32) -> Point {
        Point {
            x: clamped(self.x, lower_bound, upper_bound),
            y: self.y,
        }

    }

    #[inline]
    pub fn clamped_y(self, lower_bound: f32, upper_bound: f32) -> Point {
        Point {
            x: self.x,
            y: clamped(self.y, lower_bound, upper_bound),
        }
    }
}

// Numerical operator traits
impl Add<Point> for Point {
    type Output = Point;

    #[inline]
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Neg for Point {
    type Output = Point;

    #[inline]
    fn neg(self) -> Point {
        Point { x: -self.x , y: -self.y }
    }
}

// Conversion traits
impl From<(f32, f32)> for Point {
    #[inline]
    fn from(pt: (f32, f32)) -> Self {
        Point { x: pt.0, y: pt.1 }
    }
}

impl From<Point> for (f32, f32) {
    #[inline]
    fn from(pt: Point) -> Self {
        (pt.x, pt.y)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Rect
////////////////////////////////////////////////////////////////////////////////
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn contains(&self, pt: Point) -> bool {
        self.contains_x(pt.x) && self.contains_y(pt.y)
    }

    pub fn contains_x(&self, x: f32) -> bool {
        x >= self.left && x < self.right
    }

    pub fn contains_y(&self, y: f32) -> bool {
        y >= self.top && y < self.bottom
    }
}

