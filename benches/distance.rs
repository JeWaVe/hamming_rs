use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

unsafe fn random_vector(x: *mut u8, len: usize) {
    for i in 0..len {
        *x.add(i) = rand::random();
    }
}

fn bench_dist_aligned(c: &mut Criterion) {
    const KB: usize = 1024;
    let sizes = [
        KB / 32,
        KB / 16,
        KB / 8,
        KB / 4,
        KB / 2,
        KB,
        KB * 2,
        KB * 4,
        KB * 8,
        KB * 16,
        KB * 32,
        KB * 64,
    ];
    let mut group = c.benchmark_group("distance_aligned");
    group.warm_up_time(Duration::from_secs(1))
         .measurement_time(Duration::from_secs(2));
    for s in sizes.iter() {
        unsafe {
            let x = hamming_rs::utils::aligned_malloc(256, *s);
            random_vector(x, *s);
            let y = hamming_rs::utils::aligned_malloc(256, *s);
            random_vector(y, *s);
            let xx = std::slice::from_raw_parts(x, *s);
            let yy = std::slice::from_raw_parts(x, *s);
            group.bench_with_input(BenchmarkId::new("strsim", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(strsim::generic_hamming(data.0, data.1)))
            });
            group.bench_with_input(BenchmarkId::new("hamming_rs", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(hamming_rs::distance(data.0, data.1)))
            });
            group.bench_with_input(BenchmarkId::new("hamming", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(hamming::distance_fast(data.0, data.1)))
            });
        }
    }
    group.finish();
}

fn bench_dist_unaligned(c: &mut Criterion) {
    const KB: usize = 1024;
    let sizes = [
        KB / 32,
        KB / 16,
        KB / 8,
        KB / 4,
        KB / 2,
        KB,
        KB * 2,
        KB * 4,
        KB * 8,
        KB * 16,
        KB * 32,
        KB * 64,
    ];
    let mut group = c.benchmark_group("distance_unaligned");
    group.warm_up_time(Duration::from_secs(1))
         .measurement_time(Duration::from_secs(2));
    for s in sizes.iter() {
        unsafe {
            let mut vec_x = Vec::with_capacity(*s);
            let mut vec_y = Vec::with_capacity(*s);
            let x = vec_x.as_mut_ptr();
            random_vector(x, *s);
            let y = vec_y.as_mut_ptr();
            random_vector(y, *s);
            let xx = std::slice::from_raw_parts(x, *s);
            let yy = std::slice::from_raw_parts(x, *s);
            group.bench_with_input(BenchmarkId::new("strsim", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(strsim::generic_hamming(data.0, data.1)))
            });
            group.bench_with_input(BenchmarkId::new("hamming_rs", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(hamming_rs::distance(data.0, data.1)))
            });
            group.bench_with_input(BenchmarkId::new("hamming", s), &(xx, yy), |b, data| {
                b.iter(|| black_box(hamming::distance_fast(data.0, data.1)))
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
    ];
    let mut group = c.benchmark_group("weight");
    group.warm_up_time(Duration::from_secs(1))
         .measurement_time(Duration::from_secs(2));
    for s in sizes.iter() {
        unsafe {
            let x = hamming_rs::utils::aligned_malloc(256, *s);
            random_vector(x, *s);
            let xx = std::slice::from_raw_parts(x, *s);
            group.bench_with_input(BenchmarkId::new("hamming_rs", s), &xx, |b, data| {
                b.iter(|| black_box(hamming_rs::weight(&data)))
            });
            group.bench_with_input(BenchmarkId::new("hamming", s), &xx, |b, data| {
                b.iter(|| black_box(hamming::weight(&data)))
            });
        }
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_dist_aligned, bench_dist_unaligned, bench_weight);

criterion_main!(benches);
