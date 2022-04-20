use core::cmp::{max, min};
use std::is_x86_feature_detected;

/// Computes hamming distance (naive version)
pub fn distance_naive(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    let mut accum: u64 = 0;
    for i in 0..x.len() {
        let dist = (x[i] ^ y[i]).count_ones() as u64;
        accum += dist;
    }
    accum
}

/// computes hamming weight (naive version)
pub fn weight_naive(x: &[u8]) -> u64 {
    let mut accum: u64 = 0;
    for byte in x.iter() {
        accum += byte.count_ones() as u64;
    }
    accum
}

/// Computes hamming distance  
/// Assumes `x` and `y` have same length  
/// slightly faster than naive version  
pub fn distance_faster(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    let mut accum: u64 = 0;
    let mut i: usize = 0;
    if x.len() >= 8 {
        let x_u = x.as_ptr() as *const u64;
        let y_u = y.as_ptr() as *const u64;
        while 8 * i < x.len() - 8 {
            unsafe {
                let dist = (*x_u.add(i) ^ *y_u.add(i)).count_ones();
                accum += dist as u64;
            }
            i += 1;
        }
    }

    i *= 8;

    if i < x.len() {
        accum += distance_naive(&x[i..], &y[i..]);
    }
    accum
}

/// Computes hamming distance  
/// Assumes x and y have same memory alignment  
/// Uses highly optimized avx2 version if available  
/// fallback on [distance_faster] if `x` and `y` have different alignment or if avx2 features are not available
/// # Arguments
/// * `x` - a byte slice (prefer 32 byte alignment to get highest performance)
/// * `y` - same
///
/// # Examples
/// ```
/// use hamming_rs::distance;
/// let x: [u8;5] = [0, 1, 2, 3, 4];
/// let y: [u8;5] = [0, 1, 3, 2, 4];
/// let dist = distance(&x, &y);
/// assert_eq!(2, dist);
/// ```
pub fn distance(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return lib_avx2::distance_vect(x, y);
        }
        distance_faster(x, y)
    }
}

/// Computes hamming weight  
/// Uses highly optimized avx2 version if available
pub fn weight(x: &[u8]) -> u64 {
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return lib_avx2::weight_vect(x);
        }
        weight_naive(x)
    }
}

/// computes jaro-winkler distance - naive version
pub fn jaro_winkler(x: &[u8], y: &[u8]) -> f32 {
    let x_len = x.len();
    let y_len = y.len();
    if x_len == 0 && y_len == 0 {
        return 1.0;
    }
    if x_len == 0 || y_len == 0 {
        return 0.0;
    }
    if x_len == 1 && y_len == 1 {
        if x[0] == y[0] {
            return 1.0f32;
        }
        return 0.0f32;
    }
    let s = max(x_len, y_len) / 2 - 1;
    let mut m = 0.0f32;
    let mut t = 0.0f32;
    let mut l = 0.0f32;
    let mut i = 0;
    // identify common prefix
    loop {
        if i >= 3 || i >= x_len || i >= y_len {
            break;
        }
        if x[i] == y[i] {
            l += 1.0;
        } else {
            break;
        }
        i += 1;
    }

    let mut m_s1 = vec![];
    let mut m_s2 = vec![];
    for i in 0..x_len {
        let mut lower_bound = 0;
        if i > s {
            lower_bound = i - s;
        }
        for j in lower_bound..min(y_len, i + s) {
            if x[i] == y[j] {
                m_s1.push(x[i]);
                m_s2.push(y[j]);
                m += 1.0f32;
            }
        }
    }

    for i in 0..m_s1.len() {
        if m_s1[i] != m_s2[i] {
            t += 1.0f32;
        }
    }

    t = t * 0.5f32;

    let d_j = (m / x_len as f32 + m / y_len as f32 + (m - t) / m) / 3.0;
    d_j + 0.1f32 * l * (1.0f32 - d_j)
}

/// avx2 target specific functions
pub mod lib_avx2;

/// export utilies (such as cross platform aligned_alloc)
pub mod utils;

#[cfg(test)]
pub mod lib_test;
