// fast-itoa/benches/benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion};
use fast_itoa::rapidjson;

fn large_number_fast(criterion: &mut Criterion) {
    criterion.bench_function("print 4839532111234523", |bencher| {
        let mut dest = [0; 32];
        bencher.iter(|| {
            fast_itoa::format_u64(&mut dest, criterion::black_box(4839532111234523));
        })
    });
}

fn large_number_rapidjson(criterion: &mut Criterion) {
    criterion.bench_function("print 4839532111234523", |bencher| {
        let mut dest = [0; 32];
        bencher.iter(|| {
            unsafe {
                rapidjson::u64toa(criterion::black_box(4839532111234523), dest.as_mut_ptr());
            }
        })
    });
}

fn large_number_simple(criterion: &mut Criterion) {
    criterion.bench_function("print 4839532111234523", |bencher| {
        let mut dest = [0; 32];
        bencher.iter(|| {
            fast_itoa::format_u64_simple(&mut dest, criterion::black_box(4839532111234523));
        })
    });
}

criterion_group!(
    benches,
    large_number_fast,
    large_number_rapidjson,
    large_number_simple
);
criterion_main!(benches);
