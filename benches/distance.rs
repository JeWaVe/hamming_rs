use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

unsafe fn random_vector(x: *mut u8, len: usize) {
    for i in 0..len {
        *x.add(i) = rand::random();
    }
}

fn bench_dist(c: &mut Criterion) {
    const KB: usize = 1024;
    let sizes = [
        KB,
        KB * 2,
        KB * 4,
        KB * 8,
        KB * 16,
        KB * 32,
        KB * 64,
        KB * 128,
        KB * 256,
        KB * 512,
        KB * 1024,
    ];
    let mut group = c.benchmark_group("distance");
    for s in sizes.iter() {
        unsafe {
            let x = hamming_rs::utils::aligned_alloc(256, *s);
            random_vector(x, *s);
            let y = hamming_rs::utils::aligned_alloc(256, *s);
            random_vector(y, *s);
            let xx = std::slice::from_raw_parts(x, *s);
            let yy = std::slice::from_raw_parts(x, *s);
            group.bench_with_input(BenchmarkId::new("local", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(hamming_rs::distance(&data.0, &data.1)))
            });
            group.bench_with_input(BenchmarkId::new("reference", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(hamming::distance_fast(&data.0, &data.1)))
            });
        }
    }
    group.finish();
}

fn bench_weight(c: &mut Criterion) {
    const KB: usize = 1024;
    let sizes = [
        KB,
        KB * 2,
        KB * 4,
        KB * 8,
        KB * 16,
        KB * 32,
        KB * 64,
        KB * 128,
        KB * 256,
        KB * 512,
        KB * 1024,
    ];
    let mut group = c.benchmark_group("weight");
    for s in sizes.iter() {
        unsafe {
            let x = hamming_rs::utils::aligned_alloc(256, *s);
            random_vector(x, *s);
            let xx = std::slice::from_raw_parts(x, *s);
            group.bench_with_input(BenchmarkId::new("local", s), &xx, |b, data| {
                b.iter(|| black_box(hamming_rs::weight(&data)))
            });
            group.bench_with_input(BenchmarkId::new("reference", s), &xx, |b, data| {
                b.iter(|| black_box(hamming::weight(&data)))
            });
        }
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_dist, bench_weight);

criterion_main!(benches);
