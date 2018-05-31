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

// Module declarations.
mod line_intersect;
mod rect_intersect;

// Local imports.
use utilities::clamp;
use utilities::order;

// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;

// Exports.
pub use self::line_intersect::line_intersect;
pub use self::line_intersect::line_segment_intersect;
pub use self::rect_intersect::line_rect_intersect;

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
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    #[inline]
    pub fn one() -> Self {
        Position { x: 1, y: 1 }
    }

    #[inline]
    pub fn x_order(pair: [Position; 2]) -> [Position; 2] {
        if pair[0].x > pair[1].x {
            [pair[1], pair[0]]
        } else {
            pair
        }
    }

    #[inline]
    pub fn y_order(pair: [Position; 2]) -> [Position; 2] {
        if pair[0].y > pair[1].y {
            [pair[1], pair[0]]
        } else {
            pair
        }
    }

    #[inline]
    pub fn contained_in(self, rect: [Position; 2]) -> bool {
        let (l, r) = order(rect[0].x, rect[1].x);
        let (t, b) = order(rect[0].y, rect[1].y);

        self.x >= l && self.x <= r && self.y >= t && self.y <= b
    }

    #[inline]
    pub fn clamp_x(mut self, x1: i32, x2: i32) -> Self {
        self.x = clamp(self.x, x1, x2);
        self
    }

    #[inline]
    pub fn clamp_y(mut self, y1: i32, y2: i32) -> Self {
        self.y = clamp(self.y, y1, y2);
        self
    }
}

// Numerical operator traits
impl Add<Position> for Position {
    type Output = Position;

    #[inline]
    fn add(self, other: Position) -> Position {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    #[inline]
    fn sub(self, other: Position) -> Position {
        Position { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Neg for Position {
    type Output = Position;

    #[inline]
    fn neg(self) -> Position {
        Position { x: -self.x , y: -self.y }
    }
}

// Conversion traits
impl From<(i32, i32)> for Position {
    #[inline]
    fn from(pt: (i32, i32)) -> Self {
        Position { x: pt.0, y: pt.1 }
    }
}

impl From<Position> for (i32, i32) {
    #[inline]
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Size
////////////////////////////////////////////////////////////////////////////////
/// A 2-dimensional integral size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    #[inline]
    pub fn new(w: u32, h: u32) -> Self {
        Size { w, h }
    }
}

// Conversion traits
impl From<(u32, u32)> for Size {
    #[inline]
    fn from(pt: (u32, u32)) -> Self {
        Size { w: pt.0, h: pt.1 }
    }
}


