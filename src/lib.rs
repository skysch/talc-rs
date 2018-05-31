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
#![allow(unused)]

// Module declarations.
mod line;
mod point;
pub mod utilities;
#[cfg(test)]
mod test;

// Local imports.
use utilities::clamp;


// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;


// Exports.
pub use line::line;
pub use line::line_horizontal;
pub use line::line_vertical;
pub use line::normal_segment;
pub use line::ray;
pub use line::ray_segment;
pub use line::segment;
pub use line::segment_extended;
pub use line::segment_horizontal;
pub use line::segment_vertical;
pub use point::point;


////////////////////////////////////////////////////////////////////////////////
// Canvas
////////////////////////////////////////////////////////////////////////////////
pub trait Canvas {

    fn pixel_mut(&mut self, pos: Position) -> Option<&mut u32>;

    fn left(&self) -> i32;
    fn right(&self) -> i32;
    fn top(&self) -> i32;
    fn bottom(&self) -> i32;

    fn top_left(&self) -> Position {
        Position { x: self.left(), y: self.top() }
    }

    fn top_right(&self) -> Position {
        Position { x: self.right(), y: self.top() }
    }

    fn bottom_left(&self) -> Position {
        Position { x: self.left(), y: self.bottom() }
    }

    fn bottom_right(&self) -> Position {
        Position { x: self.right(), y: self.bottom() }
    }

    fn width(&self) -> u32 {
        (self.right() - self.left()) as u32
    }

    fn height(&self) -> u32 {
        (self.bottom() - self.top()) as u32
    }

    fn size(&self) -> Size {
        Size {
            w: self.width(), 
            h: self.height(),
        }
    }

    fn contains(&self, pos: Position) -> bool {
        self.contains_x(pos.x) && self.contains_y(pos.y)
    }

    fn contains_x(&self, x: i32) -> bool {
        x >= self.left() && x <= self.right()
    }

    fn contains_y(&self, y: i32) -> bool {
        y >= self.top() && y <= self.bottom()
    }


    fn box_clamp(&self, segment: [Position; 2]) -> Option<[Position; 2]>
    {
        let [a, b] = segment;

        Some([
            Position {
                x: clamp(a.x, self.left(), self.right()),
                y: clamp(a.y, self.top(), self.bottom()),
            },
            Position {
                x: clamp(b.x, self.left(), self.right()),
                y: clamp(b.y, self.top(), self.bottom()),
            },
        ])
    }

    fn linear_clamp(&self, segment: [Position; 2]) -> Option<[Position; 2]>
    {
        let left = self.left();
        let right = self.right();
        let top = self.top();
        let bottom = self.bottom();
            
        let [mut a, mut b] = Position::x_order(segment);
        
        // Calculate line slope.
        let rise = b.y - a.y;
        let run = b.x - a.x;

        if rise == 0 {
            // Do horizontal line checks. Simply clamping x is unsuitable in
            // case we end up clamping two external points to a shared boundary
            // point within the region. We don't want to project a completely
            // off-screen line onto the edge of the region.
            if !self.contains_y(a.y) || 
                (!self.contains_x(a.x) && !self.contains_x(b.x))
            { 
                None
            } else {
                a.x = clamp(a.x, left, right);
                b.x = clamp(b.x, left, right);
                Some([a, b])
            }

        } else if run == 0 {
            // Do vertical line checks. Simply clamping y is unsuitable in
            // case we end up clamping two external points to a shared boundary
            // point within the region. We don't want to project a completely
            // off-screen line onto the edge of the region.
            if !self.contains_x(a.x) || 
                (!self.contains_y(a.y) && !self.contains_y(b.y))
            { 
                None
            } else {
                a.y = clamp(a.y, top, bottom);
                b.y = clamp(b.y, top, bottom);
                Some([a, b])
            }

        } else {
            let slope = rise as f32 / run as f32;

            // Interpolate horizontally to the region boundaries.
            if a.x < left {
                a.y += ((left - a.x) as f32 * slope) as i32;
                a.x = left;
            }
            if b.x > right {
                b.y -= ((b.x - right) as f32 * slope) as i32;
                b.x = right;
            }

            // Interpolate vertically to the region boundaries.
            let [mut a, mut b] = Position::y_order([a, b]);
            if a.y < top {
                a.x += ((top - a.y) as f32 / slope) as i32;
                a.y = top;
            }
            if b.y > bottom {
                b.x -= ((b.y - bottom) as f32 / slope) as i32;
                b.y = bottom;
            }

            Some([a, b])
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Brush
////////////////////////////////////////////////////////////////////////////////
pub trait Brush {
    fn apply(&mut self, region: &mut Canvas, pos: Position);
}



////////////////////////////////////////////////////////////////////////////////
// Pattern
////////////////////////////////////////////////////////////////////////////////
pub trait Pattern {
    fn apply<M>(&mut self, region: &mut Canvas, mask: M)
        where M: Fn(Position) -> bool;
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


