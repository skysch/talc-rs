// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Criterion benchmarking
////////////////////////////////////////////////////////////////////////////////

// https://docs.rs/criterion/0.2.3/criterion/
#[macro_use]
extern crate criterion;
extern crate talc;


// External library imports.
use criterion::Criterion;
use talc::point;
use talc::geometry::line_intersect;
use talc::geometry::line_rect_intersect;
use talc::geometry::line_segment_intersect;
use talc::Position;
use talc::Canvas;


////////////////////////////////////////////////////////////////////////////////
// Criterion test harness setup
////////////////////////////////////////////////////////////////////////////////
criterion_main!(benches);
criterion_group!(
    name = benches;
    config = Criterion::default();
    targets =
        benchmark_point,
        benchmark_line_intersect,
        benchmark_line_segment_intersect,
        benchmark_line_rect_intersect,
);


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
        for _ in 0..len {
            buffer.push(0);
        }

        TestCanvas { buffer, stride }
    }
}

impl Canvas for TestCanvas {
    fn pixel(&self, pos: Position) -> Option<&u32> {
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= self.stride {
            return None;
        }

        let index = self.stride as i32 * pos.y + pos.x;
        
        if index < 0 || index as usize >= self.buffer.len() {
            None
        } else {
            Some(&self.buffer[index as usize])
        }
    }

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


////////////////////////////////////////////////////////////////////////////////
// Point Benchmarks
////////////////////////////////////////////////////////////////////////////////
fn benchmark_point_interior(c: &mut Criterion) {
    let mut canvas = TestCanvas::square(5);

    c.bench_function("point", move |b| b.iter(|| 
        point(&mut canvas, &mut 0xFF, Position { x: 1, y: 3 })
    ));
}

fn benchmark_point_exterior(c: &mut Criterion) {
    let mut canvas = TestCanvas::square(5);

    c.bench_function("point", move |b| b.iter(|| 
        point(&mut canvas, &mut 0xFF, Position { x: -1, y: 3 })
    ));
}

fn benchmark_point_edge(c: &mut Criterion) {
    let mut canvas = TestCanvas::square(5);

    c.bench_function("point", move |b| b.iter(|| 
        point(&mut canvas, &mut 0xFF, Position { x: 0, y: 2 })
    ));
}


////////////////////////////////////////////////////////////////////////////////
// Geometry Benchmarks
////////////////////////////////////////////////////////////////////////////////
