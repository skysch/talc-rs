// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Tests for point drawing primitives.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use canvas::Canvas;
use geometry::Point;
use primitives::segment;
use test::TestCanvas;



////////////////////////////////////////////////////////////////////////////////
// `point` tests
////////////////////////////////////////////////////////////////////////////////

#[test]
pub fn interior() {
    let mut c = TestCanvas::square(5);

    segment(&mut c, &mut 0xFF, [
        Point { x: 1.0, y: 1.0 },
        Point { x: 4.0, y: 5.0 },
    ]);

    assert_eq!(c.buffer, [
        0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ])
}

// #[test]
// pub fn exterior() {

//     let mut c = TestCanvas::square(5);

//     point(&mut c, &mut 0xFF, Point { x: -1.0, y: 4.0 });

//     assert_eq!(c.buffer, [
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//     ])
// }

// #[test]
// pub fn edge() {

//     let mut c = TestCanvas::square(5);

//     point(&mut c, &mut 0xFF, Point { x: 0.0, y: 2.0 });

//     assert_eq!(c.buffer, [
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0xFF, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00, 0x00,
//     ])
// }
