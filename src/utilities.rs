// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Common utility functions.
////////////////////////////////////////////////////////////////////////////////


#[inline]
pub fn clamp<T: PartialOrd>(val: T, lb: T, ub: T) -> T {
    if val <= lb {lb} else if val >= ub {ub} else {val}
}


#[inline]
pub fn order<T: PartialOrd>(lb: T, ub: T) -> (T, T) {
    if lb <= ub {(lb, ub)} else{(ub, lb)}
}


#[inline]
pub fn same_sign(a: i32, b: i32) -> bool {
    ((a as u32) ^ (b as u32)) as i32 >= 0
}