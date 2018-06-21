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

// Standard library imports.
use std::f32;
use std::f64;
use std::mem;
use std::ops::Add;
use std::ops::Sub;
use std::ptr;


////////////////////////////////////////////////////////////////////////////////
// same_sign
////////////////////////////////////////////////////////////////////////////////
/// Returns a `bool` indicating whether the given `f32` values have the same
/// sign.
#[inline]
pub fn same_sign(a: f32, b: f32) -> bool {
    ((a as u32) ^ (b as u32)) as i32 >= 0
}


////////////////////////////////////////////////////////////////////////////////
// sign
////////////////////////////////////////////////////////////////////////////////
/// Returns the sign of the given `f64` value as an `i8`.
#[inline]
pub fn sign(x: f64) -> i8 {
    if x < 0.0 { -1 } else if x > 0.0 { 1 } else { 0 }
}


////////////////////////////////////////////////////////////////////////////////
// close
////////////////////////////////////////////////////////////////////////////////
/// Returns true if the given `f32` values are with `precision` distance from 
/// eachother.
#[inline]
pub fn close<T>(a: T, b: T, precision: T) -> bool 
    where T: Into<f64>
{
    (a.into() - b.into()).abs() < precision.into()
}


////////////////////////////////////////////////////////////////////////////////
// nearly_equal
////////////////////////////////////////////////////////////////////////////////
/// Returns true if the given float values are nearly equal, taking into
/// account relative error and infinites.
#[inline]
pub fn nearly_equal<T>(a: T, b: T) -> bool
    where T: Into<f64>
{
    use std::f64;
    let (a, b) = (a.into(), b.into());
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b { // Shortcut, handles infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        diff < (f64::EPSILON * f64::MIN_POSITIVE)
    } else { // Use relative error.
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
    }
}


////////////////////////////////////////////////////////////////////////////////
// clamped
////////////////////////////////////////////////////////////////////////////////
/// Returns the given value clamped between the provided bounds.
#[inline]
pub fn clamped<T>(value: T, lower_bound: T, upper_bound: T) -> T
    where T: PartialOrd
{
    assert!(lower_bound <= upper_bound);
    if value < lower_bound {
        lower_bound
    } else if value > upper_bound {
        upper_bound
    } else {
        value
    }
}


////////////////////////////////////////////////////////////////////////////////
// ordered
////////////////////////////////////////////////////////////////////////////////
/// Returns the given value ordered between the provided bounds.
#[inline]
pub fn ordered<T>(a: T, b: T) -> (T, T)
    where T: PartialOrd
{
	if a <= b {(a, b)} else {(b, a)}
}


////////////////////////////////////////////////////////////////////////////////
// clipped
////////////////////////////////////////////////////////////////////////////////
/// Returns the given value pair clipped between the provided bounds if either
/// point lies between them, or `None` otherwise.
#[inline]
pub fn clipped<T>(values: (T, T), lower_bound: T, upper_bound: T)
    -> Option<(T, T)>
    where T: PartialOrd + Clone
{
    assert!(lower_bound <= upper_bound);

    if (values.0 >= lower_bound && values.0 <= upper_bound) ||
       (values.1 >= lower_bound && values.1 <= upper_bound)
    {
        Some((
            clamped(values.0, lower_bound.clone(), upper_bound.clone()),
            clamped(values.1, lower_bound, upper_bound)
        ))
    } else {
        None
    }
}


////////////////////////////////////////////////////////////////////////////////
// distance
////////////////////////////////////////////////////////////////////////////////
/// Returns the distance between the given values.
#[inline]
pub fn distance<T>(a: T, b: T) -> T where T: Sub<Output=T> + PartialOrd {
    if a > b {a - b} else {b - a}
}


////////////////////////////////////////////////////////////////////////////////
// lerp
////////////////////////////////////////////////////////////////////////////////
/// Performs a linear interpolation between `start` and `end`, returning the 
/// value located at the ratio given by `amount`, which is clamped between 0 and
/// 1.
pub fn lerp<T>(start: T, end: T, amount: f32) -> T
    where
        T: Copy + PartialOrd + Sub<T, Output=T> + Add<T, Output=T> + Into<f32>
            + From<f32>
{
    let a = if start > end {
        1.0 - clamped(amount, 0.0, 1.0)
    } else {
        clamped(amount, 0.0, 1.0)
    };

    let (s, e) = ordered(start, end);
    T::from(((e-s).into()) * a) + s
}


////////////////////////////////////////////////////////////////////////////////
// Split
////////////////////////////////////////////////////////////////////////////////
/// A type which may contain zero, one, or two of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Split<T> {
    /// No value present.
    Zero,
    /// One value present.
    One(T),
    /// Two values present.
    Two(T, T),
}

impl<T> Default for Split<T> {
    fn default() -> Self {
        Split::Zero
    }
}

impl<T> Iterator for Split<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut res = None;
        replace_with(self, |curr|
            match curr {
                Split::Zero      => {res = None;    Split::Zero}
                Split::One(v)    => {res = Some(v); Split::Zero},
                Split::Two(a, b) => {res = Some(a); Split::One(b)},
            }
        );
        res
    }
}

impl<T> From<T> for Split<T> {
    fn from(value: T) -> Self {
        Split::One(value)
    }
}

impl<T> From<(T, T)> for Split<T> {
    fn from(value: (T, T)) -> Self {
        Split::Two(value.0, value.1)
    }
}


////////////////////////////////////////////////////////////////////////////////
// replace_with
////////////////////////////////////////////////////////////////////////////////
// TODO: Replace this with std::mem::replace_with if it ever becomes 
// available.
/// Temporarily takes ownership of a value at a mutable location, and replace 
/// it with a new value based on the old one.
///
/// We move out of reference temporarily, to apply a closure, returning a new
/// value, which is then placed at the original value's location.
///
/// # An important note
///
/// The behavior on panic (or to be more precise, unwinding) is specified to
/// match the behavior of panicking inside a destructor, which itself is
/// simply specified to not unwind.
#[inline]
pub fn replace_with<T, F>(val: &mut T, replace: F)
    where F: FnOnce(T) -> T {
    // Guard against unwinding. Note that this is critical to safety, to avoid
    // the value behind the reference `val` is not dropped twice during
    // unwinding.
    let guard = ExitGuard;

    unsafe {
        // Take out the value behind the pointer.
        let old = ptr::read(val);
        // Run the closure.
        let new = replace(old);
        // Put the result back.
        ptr::write(val, new);
    }

    // Forget the guard, to avoid panicking.
    mem::forget(guard);
}

/// A guarding type which will abort upon drop.
///
/// This is used for catching unwinding and transforming it into abort.
///
/// The destructor should never be called naturally (use `mem::forget()`), and
/// only when unwinding.
struct ExitGuard;

impl Drop for ExitGuard {
    fn drop(&mut self) {
        // To avoid unwinding, we abort (we panic, which is equivalent to abort
        // inside an unwinding destructor) the program, which ensures that the
        // destructor of the invalidated value isn't runned, since this
        // destructor ought to be called only if unwinding happens.
        panic!("`replace_with` closure unwind");
    }
}
