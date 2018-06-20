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

// Internal modules.
mod angle;
mod line;

// Local imports.
use utilities::clamped;

// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
use std::f32;

// Exports.
pub use self::line::clip_line_to_rect;
pub use self::line::clip_segment_to_rect;
pub use self::line::extend_segment_to_rect;
pub use self::line::intersect_line_with_segment;
pub use self::line::intersect_segment_with_segment;
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
    /// Returns a new `Point` with the given `x`, `y` coordinates.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    /// Returns a new `Point` with coordinates `(1.0, 1.0)`.
    #[inline]
    pub fn one() -> Self {
        Point { x: 1.0, y: 1.0 }
    }
    
    /// Returns an x-ordering of the given points.
    #[inline]
    pub fn x_ordered(pair: [Point; 2]) -> [Point; 2] {
        if pair[0].x > pair[1].x {
            [pair[1], pair[0]]
        } else {
            pair
        }
    }

    /// Returns a y-ordering of the given points.
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
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<Scale> for Point {
    type Output = Point;
    fn mul(self, other: Scale) -> Point {
        Point { x: self.x * other.horz, y: self.y * other.vert }
    }
}

impl Div<Scale> for Point {
    type Output = Point;
    fn div(self, other: Scale) -> Point {
        Point { x: self.x / other.horz, y: self.y / other.vert }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}

// Conversion traits

impl From<(f32, f32)> for Point {
    fn from(pt: (f32, f32)) -> Self {
        Point { x: pt.0, y: pt.1 }
    }
}

impl From<Point> for (f32, f32) {
    fn from(pt: Point) -> Self {
        (pt.x, pt.y)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Position
////////////////////////////////////////////////////////////////////////////////
/// A point in a 2-dimensional integer plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn one() -> Self {
        Position { x: 1, y: 1 }
    }
}

// Numerical operator traits

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Position> for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Position {
        Position { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Neg for Position {
    type Output = Position;
    fn neg(self) -> Position {
        Position { x: -self.x , y: -self.y }
    }
}

// Conversion traits

impl From<(i32, i32)> for Position {
    fn from(pt: (i32, i32)) -> Self {
        Position { x: pt.0, y: pt.1 }
    }
}

impl From<Position> for (i32, i32) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Scale
////////////////////////////////////////////////////////////////////////////////
/// A 2-dimensional floating point scaling factor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    pub horz: f32,
    pub vert: f32,
}

impl Scale {
    pub fn new(horz: f32, vert: f32) -> Self {
        Scale { horz, vert }
    }
}

// Default trait

impl Default for Scale {
    fn default() -> Self {
        Scale { horz: 1.0, vert: 1.0 }
    }
}

// Numerical operator traits

impl Mul<Point> for Scale {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point { x: self.horz * other.x, y: self.vert * other.y }
    }
}

impl Div<Point> for Scale {
    type Output = Point;
    fn div(self, other: Point) -> Point {
        Point { x: self.horz / other.x, y: self.vert / other.y }
    }
}

// Conversion traits

impl From<(f32, f32)> for Scale {
    fn from(pt: (f32, f32)) -> Self {
        Scale { horz: pt.0, vert: pt.1 }
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


