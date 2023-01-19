// fast-itoa/benches/benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion};

#[inline(never)]
fn fmt_u64_simple(dest: &mut [u8; 32], mut x: u64) -> usize {
    let mut i = 0;
    while x != 0 {
        dest[i] = b'0' + (x % 10) as u8;
        x /= 10;
        i += 1;
    }
    for j in 0..(i / 2) {
        dest.swap(j, i - j - 1);
    }
    i
}

fn large_number_fast(criterion: &mut Criterion) {
    criterion.bench_function("print 4839532111234523", |bencher| {
      let mut dest = [0; 32];
      bencher.iter(|| {
        fast_itoa::fmt_u64(&mut dest, criterion::black_box(48395321));
      })
    });
}

fn large_number_simple(criterion: &mut Criterion) {
    criterion.bench_function("print 4839532111234523", |bencher| {
      let mut dest = [0; 32];
      bencher.iter(|| {
        fmt_u64_simple(&mut dest, criterion::black_box(48395321));
      })
    });
}

criterion_group!(benches, large_number_fast, large_number_simple);
criterion_main!(benches);
