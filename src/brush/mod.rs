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
    fn apply<C>(&mut self, canvas: &mut C, pt: Point, stroke: f32)
    	where C: Canvas;

    /// Returns the size of the brush.
    #[inline]
    fn size(&self) -> (u32, u32) { 
    	(1, 1)
   	}
}



// Basic brushes.
impl Brush for u32 {
	#[inline]
	fn apply<C>(&mut self, canvas: &mut C, pt: Point, _stroke: f32)
        where C: Canvas
    {
    	canvas.pixel_mut(pt).map(|p| *p = *self);
    }
}