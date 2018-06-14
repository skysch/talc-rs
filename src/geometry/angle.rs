
// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Support functions for normalizing angles.
////////////////////////////////////////////////////////////////////////////////

// Standard library imports.
use std::f64::consts::PI;


////////////////////////////////////////////////////////////////////////////////
// angle_shift
////////////////////////////////////////////////////////////////////////////////
/// Wraps and shifts the given angle into the range [base, base+2π).
#[inline]
pub fn angle_shift<T>(val: T, base: T) -> T where T: Into<f64> + From<f64> {
    let (val, base) = (val.into(), base.into());
    let wrap = 2.0 * PI;

    if val < base {
        base - (base - val) % wrap + wrap
    } else {
        base + (val - base) % wrap
    }.into()
}

////////////////////////////////////////////////////////////////////////////////
// angle_classify
////////////////////////////////////////////////////////////////////////////////
/// Classifies the given angle, returning an `AngleType` holding the result.
#[inline]
pub fn angle_classify(angle: f64) -> AngleType {
    if angle.is_infinite() || angle.is_nan() {
        return AngleType::Invalid;
    } else if angle == 0.0 {
        return AngleType::Horizontal;
    };

    let angle = angle_shift(angle, 0.0);
    if angle == 0.5 * PI {
        AngleType::Vertical
    } else {
        AngleType::Normal(angle)
    }
}

/// An angle classification.
pub enum AngleType {
    /// The angle represents a horizontal line.
    Horizontal,
    /// The angle represents a vertical line.
    Vertical,
    /// The angle represents a non-horizontal, non-vertical line.
    Normal(f64),
    /// The angle is infinite or NaN.
    Invalid,
}
