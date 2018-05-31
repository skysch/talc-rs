
// https://docs.rs/criterion/0.2.3/criterion/
#[macro_use]
extern crate criterion;
extern crate talc;

use criterion::Criterion;
use talc::utilities::line_intersect;
use talc::Position;


fn benchmark_line_intersect(c: &mut Criterion) {
    c.bench_function("line_intersect", |b| b.iter(|| 
    	line_intersect(
        	[Position::new(0, 0), Position::new(10, 0)], 
        	[Position::new(5, 5), Position::new(5, -10)])
    ));
}

criterion_group!(benches, benchmark_line_intersect);
criterion_main!(benches);