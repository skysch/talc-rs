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
use canvas::Canvas;
use geometry::Point;


////////////////////////////////////////////////////////////////////////////////
// Brush
////////////////////////////////////////////////////////////////////////////////
pub trait Brush {
    fn apply<C>(&mut self, canvas: &mut C, pt: Point)
        where C: Canvas;

    fn width(&self) -> u32 { 1 }
    fn height(&self) -> u32 { 1 }
}



// Basic brushes.
impl Brush for u32 {
	fn apply<C>(&mut self, canvas: &mut C, pt: Point)
        where C: Canvas
    {
    	canvas.pixel_mut(pt).map(|p| *p = *self);
    }
}