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
use Brush;
use Position;
use Canvas;


//////////////////////////////////////////////////////////////////////////////
// point
//////////////////////////////////////////////////////////////////////////////
/// Draws a point.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
/// `brush`: The [`Brush`] to draw with.
/// `pos`: The [`Position`] of the point.
///
pub fn point<C, B>(
    canvas: &mut C,
    brush: &mut B, 
    pos: Position)
    where
        C: Canvas,
        B: Brush
{
    brush.apply(canvas, pos);
}