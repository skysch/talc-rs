// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Drawing patterns.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use canvas::Canvas;
use geometry::Point;
use geometry::Rect;
use utilities::lerp;


////////////////////////////////////////////////////////////////////////////////
// Pattern
////////////////////////////////////////////////////////////////////////////////
/// A trait representing a fill pattern.
pub trait Pattern<X> {
    fn apply<C>(&self, canvas: &mut C, pt: Point, opacity: f32)
        where
            C: Canvas<Pixel=X>;

    /// Applies the pattern to the given canvas.
    // mask = opacity float
    fn paint<C, M>(&self, canvas: &mut C, rect: Rect, mask: M)
        where
            C: Canvas<Pixel=X>,
            M: Fn(Point) -> f32;

    /// Returns the size of the pattern.
    #[inline]
    fn size(&self) -> (u32, u32) { 
        (1, 1)
    }
}


impl Pattern<u32> for () {
    fn apply<C>(&self, _canvas: &mut C, _pt: Point, _opacity: f32)
        where
            C: Canvas<Pixel=u32>
    {
        /* Do nothing. */
    }

    fn paint<C, M>(&self, _canvas: &mut C, _rect: Rect, _mask: M)
        where
            C: Canvas<Pixel=u32>,
            M: Fn(Point) -> f32
    {
        /* Do nothing. */
    }
}

impl Pattern<u32> for u32 {
    fn apply<C>(&self, canvas: &mut C, pt: Point, opacity: f32)
        where
            C: Canvas<Pixel=u32>
    {
        canvas.aligned_pixel_mut(pt)
            .map(|p| {
                // RGBA blend.
                let bg = p.to_bytes();
                let fg = self.to_bytes();
                let blend: [u8; 4] = [
                    lerp(bg[0] as f32, fg[0] as f32, opacity) as u8,
                    lerp(bg[1] as f32, fg[1] as f32, opacity) as u8,
                    lerp(bg[2] as f32, fg[2] as f32, opacity) as u8,
                    0
                ];
                *p = u32::from_bytes(blend);
            });
    }

    fn paint<C, M>(&self, _canvas: &mut C, _rect: Rect, _mask: M)
        where
            C: Canvas<Pixel=u32>,
            M: Fn(Point) -> f32
    {
        unimplemented!()
        // canvas.each_pixel_mut(|pt, pix| if (mask)(pt) { *pix = *self });
    }
}


