use crate::utils;

unsafe fn random_data(x: *mut u8, len: usize) {
    for i in 0..len {
        *x.add(i) = rand::random();
    }
}

fn random_slice<'a>(size: usize) -> &'a [u8] {
    unsafe {
        let ptr = utils::aligned_malloc(64, size);
        random_data(ptr, size);
        return std::slice::from_raw_parts(ptr, size);
    }
}

#[test]
pub fn test_dist_naive() {
    let x = random_slice(12);
    let y = random_slice(12);
    let a = super::distance_naive(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_weight_naive() {
    let x = random_slice(12);
    let a = super::weight_naive(&x);
    let b = hamming::weight(&x);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_faster() {
    let x = random_slice(12);
    let y = random_slice(12);
    let a = super::distance_faster(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_faster_2() {
    let x = random_slice(16);
    let y = random_slice(16);
    let a = super::distance_faster(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_faster_3() {
    let x = random_slice(4);
    let y = random_slice(4);
    let a = super::distance_faster(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_faster_4() {
    let x = random_slice(32);
    let y = random_slice(32);
    let a = super::distance_faster(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_avx() {
    let x = random_slice(1024);
    let y = random_slice(1024);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_weight_avx() {
    let x = random_slice(1024);
    let a = super::weight(&x);
    let b = hamming::weight(&x);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_avx_2() {
    let x = random_slice(12);
    let y = random_slice(12);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_weight_avx_2() {
    let x = random_slice(12);
    let a = super::weight(&x);
    let b = hamming::weight(&x);
    assert_eq!(a, b);
}

#[test]
pub fn test_dist_avx_3() {
    let x = random_slice(128);
    let y = random_slice(128);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_weight_avx_3() {
    let x = random_slice(128);
    let a = super::weight(&x);
    let b = hamming::weight(&x);
    assert_eq!(a, b);
}

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
