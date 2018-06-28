// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Point drawing primitives.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use brush::Brush;
use canvas::Canvas;
use geometry::Point;


//////////////////////////////////////////////////////////////////////////////
// point
//////////////////////////////////////////////////////////////////////////////
/// Draws a point.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
/// `brush`: The [`Brush`] to draw with.
/// `pt`: The [`Point`] of the point.
#[inline]
pub fn point<C, B, X>(
    canvas: &mut C,
    brush: &B, 
    pt: Point)
    where
        C: Canvas<Pixel=X>,
        B: Brush<X>
{
    brush.apply(canvas, pt);
}