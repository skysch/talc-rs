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

// Module declarations.
mod geometry;
mod point;
mod line;

// Local imports.
use canvas::Canvas;
use geometry::Point;




////////////////////////////////////////////////////////////////////////////////
// TestCanvas
////////////////////////////////////////////////////////////////////////////////

pub struct TestCanvas {
    pub buffer: Vec<u32>,
    pub stride: usize,
}

impl TestCanvas {
    fn square(stride: usize) -> Self {
        let len = stride * stride;
        let mut buffer = Vec::with_capacity(len);
        for i in 0..len {
            buffer.push(0);
        }

        TestCanvas { buffer, stride }
    }
}


impl Canvas for TestCanvas {
    fn pixel(&self, pt: Point) -> Option<&u32> {
        let (x, y) = (pt.x.floor() as i32, pt.y.floor() as i32);

        if x < 0 || y < 0 || x as usize >= self.stride {
            return None;
        }

        let index = self.stride as i32 * y + x;

        if index < 0 || index as usize >= self.buffer.len() {
            None
        } else {
            Some(&self.buffer[index as usize])
        }
    }

    fn pixel_mut(&mut self, pt: Point) -> Option<&mut u32> {
        let (x, y) = (pt.x.floor() as i32, pt.y.floor() as i32);

        if x < 0 || y < 0 || x as usize >= self.stride {
            return None;
        }

        let index = self.stride as i32 * y + x;

        if index < 0 || index as usize >= self.buffer.len() {
            None
        } else {
            Some(&mut self.buffer[index as usize])
        }
    }

    fn left(&self) -> f32 { 0.0 }

    fn right(&self) -> f32 { self.stride as f32 - 1.0 }

    fn top(&self) -> f32 { 0.0 }

    fn bottom(&self) -> f32 {
        debug_assert_eq!(self.buffer.len() % (self.stride as usize), 0);
        ((self.buffer.len() / (self.stride as usize)) as f32) - 1.0
    }
}