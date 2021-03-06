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

/// avx2 target specific functions
pub mod lib_avx2;

/// export utilies (such as cross platform aligned_alloc)
pub mod utils;

#[cfg(test)]
pub mod lib_test;
