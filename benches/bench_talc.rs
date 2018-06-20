
#[macro_use]
extern crate criterion;
extern crate talc;

use criterion::Criterion;
use criterion::Fun;

use talc::primitive;
use talc::Point;
use talc::Canvas;

criterion_group!(benches, 
	draw_point,
	draw_segment,
	draw_segment_horizontal,
	draw_segment_vertical);

criterion_main!(benches);


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
        for i in 0..len {
            buffer.push(0);
        }

        TestCanvas { buffer, stride }
    }
}


impl Canvas for TestCanvas {
    fn aligned_pixel(&self, pt: Point) -> Option<&u32> {
        let (x, y) = (pt.x.floor() as i32, pt.y.floor() as i32);

        if x < 0 || y < 0 || x as usize >= self.stride {
            return None;
        }

        let index = self.stride as i32 * y + x;

        if index < 0 || index as usize >= self.buffer.len() {
            None
        } else {
            Some(&self.buffer[index as usize])
        }
    }

    fn aligned_pixel_mut(&mut self, pt: Point) -> Option<&mut u32> {
        let (x, y) = (pt.x.floor() as i32, pt.y.floor() as i32);

        if x < 0 || y < 0 || x as usize >= self.stride {
            return None;
        }

        let index = self.stride as i32 * y + x;

        if index < 0 || index as usize >= self.buffer.len() {
            None
        } else {
            Some(&mut self.buffer[index as usize])
        }
    }

    fn left(&self) -> f32 { 0.0 }

    fn right(&self) -> f32 { self.stride as f32 - 1.0 }

    fn top(&self) -> f32 { 0.0 }

    fn bottom(&self) -> f32 {
        debug_assert_eq!(self.buffer.len() % (self.stride as usize), 0);
        ((self.buffer.len() / (self.stride as usize)) as f32) - 1.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Benchmarks
////////////////////////////////////////////////////////////////////////////////

fn draw_point(crit: &mut Criterion) {
	let interior = Point { x: 910.0, y: 820.1 };
	let exterior = Point { x: -910.0, y: -820.1 };
	
	let mut canvas = TestCanvas::square(1000);
    let a = Fun::new("Interior", move |b, i| b.iter(|| 
    		primitive::point(&mut canvas, &mut 0xFFu32, interior)));

	let mut canvas = TestCanvas::square(1000);
    let b = Fun::new("Exterior", move |b, i| b.iter(|| 
    		primitive::point(&mut canvas, &mut 0xFFu32, exterior)));
    
    let functions = vec![a, b];
    crit.bench_functions("Draw Point", functions, &0);
}


fn draw_segment(crit: &mut Criterion) {
	let interior = [Point { x: 910.0, y: 820.1 }, Point { x: 40.0, y: 55.6 }];
	let exterior = [Point { x: -910.0, y: -820.1 }, Point { x: -40.0, y: -55.6 }];
	let horizontal = [Point { x: -10.0, y: 50.0 }, Point { x: 1100.0, y: 50.0 }];
	let vertical = [Point { x: 50.0, y: -50.0 }, Point { x: 50.0, y: 1150.0 }];
	let point = [Point { x: 10.0, y: 10.0 }, Point { x: 10.0, y: 10.0 }];

	
	let mut canvas = TestCanvas::square(1000);
    let a = Fun::new("Interior", move |b, i| b.iter(|| 
    		primitive::segment(&mut canvas, &mut 0xFFu32, interior)));

	let mut canvas = TestCanvas::square(1000);
    let b = Fun::new("Exterior", move |b, i| b.iter(|| 
    		primitive::segment(&mut canvas, &mut 0xFFu32, exterior)));

	let mut canvas = TestCanvas::square(1000);
    let c = Fun::new("Horizontal", move |b, i| b.iter(|| 
    		primitive::segment(&mut canvas, &mut 0xFFu32, horizontal)));

	let mut canvas = TestCanvas::square(1000);
    let d = Fun::new("Vertical", move |b, i| b.iter(|| 
    		primitive::segment(&mut canvas, &mut 0xFFu32, vertical)));

	let mut canvas = TestCanvas::square(1000);
    let e = Fun::new("Point", move |b, i| b.iter(|| 
    		primitive::segment(&mut canvas, &mut 0xFFu32, point)));
    
    let functions = vec![a, b, c, d, e];
    crit.bench_functions("Draw Segment", functions, &0);
}

fn draw_segment_horizontal(crit: &mut Criterion) {
	let mut canvas = TestCanvas::square(1000);
    let a = Fun::new("Interior", move |b, i| b.iter(|| 
    		primitive::segment_horizontal(&mut canvas, &mut 0xFFu32, 
    			Point { x: 100.0, y: 15.0 }, 920.0 )));

	let mut canvas = TestCanvas::square(1000);
    let b = Fun::new("Exterior", move |b, i| b.iter(|| 
    		primitive::segment_horizontal(&mut canvas, &mut 0xFFu32,
    			Point { x: 100.0, y: -15.0 }, 920.0 )));

	let mut canvas = TestCanvas::square(1000);
    let c = Fun::new("Point", move |b, i| b.iter(|| 
    		primitive::segment_horizontal(&mut canvas, &mut 0xFFu32, 
    			Point { x: 100.0, y: 15.0 }, 100.0 )));
    
    let functions = vec![a, b, c];
    crit.bench_functions("Draw Segment Horizontal", functions, &0);
}

fn draw_segment_vertical(crit: &mut Criterion) {
	let mut canvas = TestCanvas::square(1000);
    let a = Fun::new("Interior", move |b, i| b.iter(|| 
    		primitive::segment_vertical(&mut canvas, &mut 0xFFu32, 
    			Point { x: 15.0, y: 100.0 }, 920.0 )));

	let mut canvas = TestCanvas::square(1000);
    let b = Fun::new("Exterior", move |b, i| b.iter(|| 
    		primitive::segment_vertical(&mut canvas, &mut 0xFFu32,
    			Point { x: -15.0, y: 100.0 }, 920.0 )));

	let mut canvas = TestCanvas::square(1000);
    let c = Fun::new("Point", move |b, i| b.iter(|| 
    		primitive::segment_vertical(&mut canvas, &mut 0xFFu32, 
    			Point { x: 15.0, y: 100.0 }, 100.0 )));
    
    let functions = vec![a, b, c];
    crit.bench_functions("Draw Segment Vertical", functions, &0);
}
