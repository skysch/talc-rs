// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Drawing primitives.
////////////////////////////////////////////////////////////////////////////////

// Internal modules.
mod line;
mod point;
mod text;

// Exports.
pub use self::line::line;
pub use self::line::line_horizontal;
pub use self::line::line_vertical;
pub use self::line::normal_segment;
pub use self::line::segment;
pub use self::line::segment_extended;
pub use self::line::segment_horizontal;
pub use self::line::segment_vertical;
pub use self::point::point;
pub use self::text::FontStyle;
pub use self::text::glyph;
pub use self::text::prepare_glyph;
pub use self::text::prepare_text;
pub use self::text::PreparedText;
pub use self::text::text;
