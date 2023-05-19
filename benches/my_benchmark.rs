use criterion::{black_box, Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("happynum 1 million", |b| b.iter(|| happynum::happynum::count_distinct_happy_numbers_in_range(black_box(1000000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);