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
use brush::Brush;
use geometry::Rect;
use geometry::Point;
use utilities::clamped;


////////////////////////////////////////////////////////////////////////////////
// Canvas
////////////////////////////////////////////////////////////////////////////////
/// A trait representing the drawing surface.
pub trait Canvas {

    ////////////////////////////////////////////////////////////////////////////
    // Accessors
    ////////////////////////////////////////////////////////////////////////////

    fn pixel(&self, pt: Point) -> Option<&u32>;
    fn pixel_mut(&mut self, pt: Point) -> Option<&mut u32>;

    fn left(&self) -> f32;
    fn top(&self) -> f32;
    fn right(&self) -> f32;
    fn bottom(&self) -> f32;


    ////////////////////////////////////////////////////////////////////////////
    // Point accessors
    ////////////////////////////////////////////////////////////////////////////
    fn top_left(&self) -> Point {
        Point { x: self.left(), y: self.top() }
    }

    fn top_right(&self) -> Point {
        Point { x: self.right(), y: self.top() }
    }

    fn bottom_left(&self) -> Point {
        Point { x: self.left(), y: self.bottom() }
    }

    fn bottom_right(&self) -> Point {
        Point { x: self.right(), y: self.bottom() }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Segment accessors
    ////////////////////////////////////////////////////////////////////////////
    fn left_segment(&self) -> [Point; 2] {
        [self.top_left(), self.bottom_left()]
    }

    fn top_segment(&self) -> [Point; 2] {
        [self.top_left(), self.top_left()]
    }

    fn right_segment(&self) -> [Point; 2] {
        [self.top_right(), self.bottom_right()]
    }

    fn bottom_segment(&self) -> [Point; 2] {
        [self.bottom_right(), self.bottom_right()]
    }

    fn main_diagonal(&self) -> [Point; 2] {
        [self.top_left(), self.bottom_right()]
    }

    fn cross_diagonal(&self) -> [Point; 2] {
        [self.top_right(), self.bottom_left()]
    }

    ////////////////////////////////////////////////////////////////////////////
    // Size accessors
    ////////////////////////////////////////////////////////////////////////////

    fn width(&self) -> f32 {
        self.right() - self.left()
    }

    fn height(&self) -> f32 {
        self.bottom() - self.top()
    }

    /// A `Rect` indicating the `Canvas`'s boundaries.
    fn bounding_rect(&self) -> Rect {
        Rect {
            left: self.left(),
            top: self.top(),
            right: self.right(),
            bottom: self.bottom(),
        }
    }

    /// The `Canvas`'s boundaries expanded by the size of the given `Brush`.
    fn virtual_bounding_rect<B>(&self, brush: &B) -> Rect where B: Brush {
        let bw = brush.width() as f32;
        let bh = brush.height() as f32;

        Rect {
            left: self.left() - bw,
            top: self.top() - bh,
            right: self.right() + bw,
            bottom: self.bottom() + bh,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Queries
    ////////////////////////////////////////////////////////////////////////////

    fn contains(&self, pt: Point) -> bool {
        self.contains_x(pt.x) && self.contains_y(pt.y)
    }

    fn contains_x(&self, x: f32) -> bool {
        x >= self.left() && x < self.right()
    }

    fn contains_y(&self, y: f32) -> bool {
        y >= self.top() && y < self.bottom()
    }
}
