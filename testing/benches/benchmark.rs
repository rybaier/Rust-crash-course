
#![allow(unused)]
fn main() {
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh 8,9,10", |b| b.iter(|| sploosh(8,9,10)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
}

// report fran .37s with black_box on each argument sploosh(black_box(8), black_box(9), black_box(10)
// report ran .27s without black_box on each argument sploosh(8,9,10)cd 