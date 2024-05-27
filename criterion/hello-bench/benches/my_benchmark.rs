use criterion::{black_box, Criterion, criterion_group, criterion_main};
use hello_bench::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);