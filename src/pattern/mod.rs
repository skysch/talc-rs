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
// Pattern
////////////////////////////////////////////////////////////////////////////////
pub trait Pattern {
    fn apply<C, M>(&mut self, canvas: &mut C, mask: M)
        where
            C: Canvas,
            M: Fn(Point) -> bool;
}


// Basic patterns.
impl Pattern for u32 {
	fn apply<C, M>(&mut self, canvas: &mut C, mask: M)
        where
            C: Canvas,
            M: Fn(Point) -> bool
    {
        unimplemented!()
    	// canvas.each_pixel_mut(|pt, pix| if (mask)(pt) { *pix = *self });
    }
}