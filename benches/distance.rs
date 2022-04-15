use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn random_vector(x: &mut [u8]) {
    for i in 0..x.len() {
        x[i] = rand::random()
    }
}

extern "C" {
    fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
}

unsafe fn aligned_vector(s: usize) -> Vec<u8> {
    let ptr = aligned_alloc(64, s);
    //println!("before : {:?}", ptr);
    let result = std::vec::Vec::from_raw_parts(ptr, s, s);
    //println!("after  : {:?}", result.as_ptr());

    result
}

fn bench_fibs(c: &mut Criterion) {
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
        KB * 2048,
        KB * 4096,
        KB * 8192,
        KB * 65536,
    ];
    let mut group = c.benchmark_group("distance");
    for s in sizes.iter() {
        unsafe {
            let mut x = aligned_vector(*s);
            random_vector(&mut x);
            let mut y = aligned_vector(*s);
            random_vector(&mut y);
            group.bench_with_input(
                BenchmarkId::new("local", s),
                &(x.clone(), y.clone()),
                |b, data| b.iter(|| black_box(hamming_rs::distance(&data.0, &data.1))),
            );
            group.bench_with_input(
                BenchmarkId::new("reference", s),
                &(x.clone(), y.clone()),
                |b, data| b.iter(|| black_box(hamming::distance_fast(&data.0, &data.1))),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
