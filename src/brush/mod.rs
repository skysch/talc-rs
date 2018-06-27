// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Drawing brushes.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use canvas::Canvas;
use geometry::Point;


////////////////////////////////////////////////////////////////////////////////
// Brush
////////////////////////////////////////////////////////////////////////////////
/// A trait representing a brush stroke.
pub trait Brush {
	/// Applies the brush to the given canvas.
    fn apply<C>(&self, canvas: &mut C, pt: Point)
    	where C: Canvas;

    fn stroke<C>(&self, canvas: &mut C, vertices: &[Point])
        where C: Canvas;

    /// Returns the size of the brush.
    #[inline]
    fn size(&self) -> (u32, u32) { 
    	(1, 1)
   	}
}



// Basic brushes.
impl Brush for () {
    #[inline]
    fn apply<C>(&self, _canvas: &mut C, _pt: Point)
        where C: Canvas
    {
        /* Do nothing. */
    }

    #[inline]
    fn stroke<C>(&self, _canvas: &mut C, _vertices: &[Point])
        where C: Canvas
    {
        /* Do nothing. */
    }
}

impl Brush for u32 {
	#[inline]
	fn apply<C>(&self, canvas: &mut C, pt: Point)
        where C: Canvas
    {
    	canvas.aligned_pixel_mut(pt).map(|p| *p = *self);
    }

    #[inline]
    fn stroke<C>(&self, _canvas: &mut C, _vertices: &[Point])
        where C: Canvas
    {
        unimplemented!()
    }
}

