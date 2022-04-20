use crate::utils;
use rand::{distributions::Alphanumeric, Rng}; // 0.8

unsafe fn random_data(x: *mut u8, len: usize) {
    for i in 0..len {
        *x.add(i) = rand::random();
    }
}

fn random_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn random_slice<'a>(size: usize) -> &'a [u8] {
    unsafe {
        let ptr = utils::aligned_alloc(64, size);
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

#[test]
pub fn test_jaro_winkler() {
    for _ in 0..100 {
        let x = random_str(12);
        let y = random_str(12);
        let a = super::jaro_winkler(x.as_bytes(), y.as_bytes());
        let b = strsim::jaro_winkler(&x, &y) as f32;
        let mut b_a = b - a;
        if b_a < 0.0f32 {
            b_a = -b_a;
        }
        assert_eq!(a, b);
    }
}
