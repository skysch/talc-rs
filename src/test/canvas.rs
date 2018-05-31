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

// Local imports.
use canvas::Canvas;
use geometry::Position;
use test::TestCanvas;

////////////////////////////////////////////////////////////////////////////////
// 
////////////////////////////////////////////////////////////////////////////////


#[test]
pub fn pixel() {
    let mut c = TestCanvas::square(5);

    c.pixel_mut(Position::new(3, 4)).map(|p| *p= 0xAA);
    c.pixel_mut(Position::new(0, 0)).map(|p| *p= 0xBB);
    c.pixel_mut(Position::new(4, 4)).map(|p| *p= 0xCC);
    c.pixel_mut(Position::new(0, 4)).map(|p| *p= 0xDD);
    c.pixel_mut(Position::new(4, 0)).map(|p| *p= 0xEE);
    c.pixel_mut(Position::new(5, 0)).map(|p| *p= 0xFF);
    c.pixel_mut(Position::new(0, 5)).map(|p| *p= 0xFF);
    c.pixel_mut(Position::new(0, -10)).map(|p| *p= 0xFF);

    assert_eq!(c.buffer, vec![
        0xBB, 0x00, 0x00, 0x00, 0xEE,
        0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
        0xDD, 0x00, 0x00, 0xAA, 0xCC,
    ])
}

#[test]
pub fn box_clamp() {
    let mut c = TestCanvas::square(5);

    let segment = [Position::new(1, -1), Position::new(5, 1)];
    let segment = c.box_clamp(segment).unwrap();

    assert_eq!(segment, [Position::new(1, 0), Position::new(4, 1)])
}

#[test]
pub fn linear_clamp_45_positive() {
    let mut c = TestCanvas::square(5);

    let segment = [Position::new(-2, -2), Position::new(6, 6)];
    let segment = c.linear_clamp(segment).unwrap();

    assert_eq!(segment, [Position::new(0, 0), Position::new(4, 4)])
}

#[test]
pub fn linear_clamp_45_positive_reversed() {
    let mut c = TestCanvas::square(5);

    let segment = [Position::new(6, 6), Position::new(-2, -2)];
    let segment = c.linear_clamp(segment).unwrap();

    assert_eq!(segment, [Position::new(0, 0), Position::new(4, 4)])
}

#[test]
pub fn linear_clamp_45_negative() {
    let mut c = TestCanvas::square(5);

    let segment = [Position::new(-2, 6), Position::new(6, -2)];
    let segment = c.linear_clamp(segment).unwrap();

    assert_eq!(segment, [Position::new(4, 0), Position::new(0, 4)])
}

#[test]
pub fn linear_clamp_45_negative_reversed() {
    let mut c = TestCanvas::square(5);

    let segment = [Position::new(6, -2), Position::new(-2, 6)];
    let segment = c.linear_clamp(segment).unwrap();

    assert_eq!(segment, [Position::new(4, 0), Position::new(0, 4)])
}
