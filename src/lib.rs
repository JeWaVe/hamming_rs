use std::is_x86_feature_detected;

/// Computes hamming distance (naive version)
/// copied from [hamming bitwise fast crate](https://github.com/emschwartz/hamming-bitwise-fast/) as author agreed
pub fn distance_naive(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());

    // Process 8 bytes at a time using u64
    let mut distance = x
        .chunks_exact(8)
        .zip(y.chunks_exact(8))
        .map(|(x_chunk, y_chunk)| {
            // This is safe because we know the chunks are exactly 8 bytes.
            // Also, we don't care whether the platform uses little-endian or big-endian
            // byte order. Since we're only XORing values, we just care that the
            // endianness is the same for both.
            let x_val = u64::from_ne_bytes(x_chunk.try_into().unwrap());
            let y_val = u64::from_ne_bytes(y_chunk.try_into().unwrap());
            (x_val ^ y_val).count_ones() as u64
        })
        .sum::<u64>();

    if x.len() % 8 != 0 {
        distance += x
            .chunks_exact(8)
            .remainder()
            .iter()
            .zip(y.chunks_exact(8).remainder())
            .map(|(x_byte, y_byte)| (x_byte ^ y_byte).count_ones() as u64)
            .sum::<u64>();
    }

    distance
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
/// Assumes x and y have same memory alignment  
/// Uses highly optimized avx2 version if available  
/// fallback on [distance_naive] if `x` and `y` have different alignment or if avx2 features are not available
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
        // from benchmarks, it seems avx2 implementation is slower than naive for very small vectors
        if is_x86_feature_detected!("avx2") && x.len() >= 1024 {
            return lib_avx2::distance_vect(x, y);
        }

        distance_naive(x, y)
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
