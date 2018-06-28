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
pub trait Brush<X> {
	/// Applies the brush to the given canvas.
    fn apply<C>(&self, canvas: &mut C, pt: Point)
    	where C: Canvas<Pixel=X>;

    fn stroke<C>(&self, canvas: &mut C, vertices: &[Point])
        where C: Canvas<Pixel=X>;

    /// Returns the size of the brush.
    #[inline]
    fn size(&self) -> (u32, u32) { 
    	(1, 1)
   	}
}



// Basic brushes.
impl Brush<u32> for () {
    #[inline]
    fn apply<C>(&self, _canvas: &mut C, _pt: Point)
        where C: Canvas<Pixel=u32>
    {
        /* Do nothing. */
    }

    #[inline]
    fn stroke<C>(&self, _canvas: &mut C, _vertices: &[Point])
        where C: Canvas<Pixel=u32>
    {
        /* Do nothing. */
    }
}

impl Brush<u32> for u32 {
	#[inline]
	fn apply<C>(&self, canvas: &mut C, pt: Point)
        where C: Canvas<Pixel=u32>
    {
    	canvas.aligned_pixel_mut(pt).map(|p| *p = *self);
    }

    #[inline]
    fn stroke<C>(&self, _canvas: &mut C, _vertices: &[Point])
        where C: Canvas<Pixel=u32>
    {
        unimplemented!()
    }
}

