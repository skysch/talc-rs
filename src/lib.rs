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
#![allow(unused)]

// Module declarations.
mod canvas;
mod geometry;
mod line;
mod point;
pub mod utilities;
#[cfg(test)]
mod test;

// Local imports.
use utilities::clamp;
use utilities::order;


// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;


// Exports.
pub use canvas::Canvas;
pub use geometry::Position;
pub use geometry::Size;
pub use line::line;
pub use line::line_horizontal;
pub use line::line_vertical;
pub use line::normal_segment;
pub use line::ray;
pub use line::ray_segment;
pub use line::segment;
pub use line::segment_extended;
pub use line::segment_horizontal;
pub use line::segment_vertical;
pub use point::point;


////////////////////////////////////////////////////////////////////////////////
// Brush
////////////////////////////////////////////////////////////////////////////////
pub trait Brush {
    fn apply(&mut self, region: &mut Canvas, pos: Position);
}



////////////////////////////////////////////////////////////////////////////////
// Pattern
////////////////////////////////////////////////////////////////////////////////
pub trait Pattern {
    fn apply<M>(&mut self, region: &mut Canvas, mask: M)
        where M: Fn(Position) -> bool;
}


