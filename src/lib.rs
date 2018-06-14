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

// Public modules.
pub mod geometry;

// Internal modules.
mod brush;
mod canvas;
mod line;
mod pattern;
mod point;
mod utilities;

#[cfg(test)]
mod test;

// Standard library imports.
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;


// Exports.
pub use brush::Brush;
pub use canvas::Canvas;
pub use geometry::Rect;
pub use geometry::Point;
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
pub use pattern::Pattern;
pub use point::point;






