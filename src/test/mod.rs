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
mod canvas;
mod utilities;

// Local imports.
use canvas::Canvas;
use geometry::Position;

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
    fn pixel_mut(&mut self, pos: Position) -> Option<&mut u32> {
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= self.stride {
            return None;
        }

        let index = self.stride as i32 * pos.y + pos.x;

        
        if index < 0 || index as usize >= self.buffer.len() {
            None
        } else {
            Some(&mut self.buffer[index as usize])
        }
    }

    fn left(&self) -> i32 { 0 }
    
    fn right(&self) -> i32 { self.stride as i32 - 1 }
    
    fn top(&self) -> i32 { 0 }

    fn bottom(&self) -> i32 {
        debug_assert_eq!(self.buffer.len() % (self.stride as usize), 0);
        ((self.buffer.len() / (self.stride as usize)) as i32) - 1
    }
}