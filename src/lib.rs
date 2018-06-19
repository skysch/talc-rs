// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Drawing and geometry primitives.
//! 
//! All Talc drawing functions take a `Canvas` and a tool. The function will
//! usually attempt to clip the drawn figure to the canvas area in order to
//! avoid drawing outside of the canvas boundaries. (The canvas boundaries are
//! extended by the size of the tool in order to avoid edge discontinuities.)
//! 
//! After clipping and calculating the figure, the figure's draw points will be
//! passed to the tool, which must translate them into canvas coordinates and do
//! the drawing.
//! 
////////////////////////////////////////////////////////////////////////////////

// Public modules.
pub mod geometry;
pub mod primitive;

// Internal modules.
mod brush;
mod canvas;
// mod pattern;

#[allow(unused)]
mod utilities;

#[cfg(test)]
mod test;


// Exports.
// pub use pattern::Pattern;
pub use brush::Brush;
pub use canvas::Canvas;
pub use geometry::Point;
pub use geometry::Rect;






