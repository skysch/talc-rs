// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Drawing canvas interface.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use brush::Brush;
use geometry::Rect;
use geometry::Point;


////////////////////////////////////////////////////////////////////////////////
// Canvas
////////////////////////////////////////////////////////////////////////////////
/// A trait representing the drawing surface.
///
/// The `Canvas`'s is reponsible for mapping the continuous 'object' space
/// coordinates to the discrete coordinates used by the underlying buffer or
/// texture. Object coordinates are represented by `f32` values which are scaled
/// identically to the underlying discrete coordinates. I.e., the coordinate
/// (x, y) in object space corresponds to the upper left of the pixel at (x, y)
/// in the buffer.
pub trait Canvas {

    ////////////////////////////////////////////////////////////////////////////
    // Accessors
    ////////////////////////////////////////////////////////////////////////////

    /// Returns a reference to the pixel at the given [`Point`], or `None` if
    /// the `Point` lies outside the canvas boundary.
    ///
    /// [`Point`]: geometry/struct.Point.html
    fn pixel(&self, pt: Point) -> Option<&u32>;

    /// Returns a mutable reference to the pixel at the given [`Point`], or
    /// `None` if the `Point` lies outside the canvas boundary.
    ///
    /// [`Point`]: geometry/struct.Point.html
    fn pixel_mut(&mut self, pt: Point) -> Option<&mut u32>;

    /// Returns the (inclusive) left boundary coordinate of the canvas.
    fn left(&self) -> f32;

    /// Returns the (inclusive) top boundary coordinate of the canvas.
    fn top(&self) -> f32;

    /// Returns the (exclusive) right boundary coordinate of the canvas.
    fn right(&self) -> f32;

    /// Returns the (exclusive) bottom boundary coordinate of the canvas.
    fn bottom(&self) -> f32;

    ////////////////////////////////////////////////////////////////////////////
    // Point accessors
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the top-left [`Point`] of the `Canvas`es boundary.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn top_left(&self) -> Point {
        Point { x: self.left(), y: self.top() }
    }

    /// Returns the top-right [`Point`] of the `Canvas`es boundary.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn top_right(&self) -> Point {
        Point { x: self.right(), y: self.top() }
    }

    /// Returns the bottom-left [`Point`] of the `Canvas`es boundary.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn bottom_left(&self) -> Point {
        Point { x: self.left(), y: self.bottom() }
    }

    /// Returns the bottom-right [`Point`] of the `Canvas`es boundary.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn bottom_right(&self) -> Point {
        Point { x: self.right(), y: self.bottom() }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Segment accessors
    ////////////////////////////////////////////////////////////////////////////
    /// Returns the endpoints of the line segment along the `Canvas`es left
    /// boundary.
    #[inline]
    fn left_segment(&self) -> [Point; 2] {
        [self.top_left(), self.bottom_left()]
    }

    /// Returns the endpoints of the line segment along the `Canvas`es top
    /// boundary.
    #[inline]
    fn top_segment(&self) -> [Point; 2] {
        [self.top_left(), self.top_left()]
    }

    /// Returns the endpoints of the line segment along the `Canvas`es right
    /// boundary.
    #[inline]
    fn right_segment(&self) -> [Point; 2] {
        [self.top_right(), self.bottom_right()]
    }

    /// Returns the endpoints of the line segment along the `Canvas`es bottom
    /// boundary.
    #[inline]
    fn bottom_segment(&self) -> [Point; 2] {
        [self.bottom_right(), self.bottom_right()]
    }

    /// Returns the endpoints of the line segment along the `Canvas`es main
    /// (top-left to bottom-right) diagonal.
    #[inline]
    fn main_diagonal(&self) -> [Point; 2] {
        [self.top_left(), self.bottom_right()]
    }

    /// Returns the endpoints of the line segment along the `Canvas`es cross
    /// (top-right to bottom-left) diagonal.
    #[inline]
    fn cross_diagonal(&self) -> [Point; 2] {
        [self.top_right(), self.bottom_left()]
    }

    ////////////////////////////////////////////////////////////////////////////
    // Size accessors
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the width of the `Canvas`es boundary region.
    #[inline]
    fn width(&self) -> f32 {
        self.right() - self.left()
    }

    /// Returns the height of the `Canvas`es boundary region.
    #[inline]
    fn height(&self) -> f32 {
        self.bottom() - self.top()
    }

    /// A [`Rect`] indicating all of the `Canvas`'s boundaries.
    ///
    /// [`Rect`] geometry/struct.rect.html
    #[inline]
    fn bounding_rect(&self) -> Rect {
        Rect {
            left: self.left(),
            top: self.top(),
            right: self.right(),
            bottom: self.bottom(),
        }
    }

    /// Returns the `Canvas`'s boundaries expanded by the size of the given
    /// [`Brush`].
    ///
    /// [`Brush`]: brush/trait.brush.html
    #[inline]
    fn virtual_bounding_rect<B>(&self, brush: &B) -> Rect where B: Brush {
        let (w, h) = brush.size();
        let (bw, bh) =  (w as f32, h as f32);

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
    
    /// Returns `true` if  the given [`Point`] lies within the `Canvas`es
    /// boundaries.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn contains(&self, pt: Point) -> bool {
        self.contains_x(pt.x) && self.contains_y(pt.y)
    }

    /// Returns `true` if  the given [`Point`] lies within the `Canvas`es 
    /// horizontal boundaries.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn contains_x(&self, x: f32) -> bool {
        x >= self.left() && x < self.right()
    }

    /// Returns `true` if  the given [`Point`] lies within the `Canvas`es 
    /// vertical boundaries.
    ///
    /// [`Point`]: geometry/struct.Point.html
    #[inline]
    fn contains_y(&self, y: f32) -> bool {
        y >= self.top() && y < self.bottom()
    }
}
