use std::is_x86_feature_detected;

pub(crate) fn distance_naive(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    let mut accum: u64 = 0;
    for i in 0..x.len() {
        let dist = (x[i] ^ y[i]).count_ones() as u64;
        accum += dist;
    }
    accum
}

pub(crate) unsafe fn distance_faster(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    let mut accum: u64 = 0;
    let mut i: usize = 0;
    if x.len() >= 8 {
        let x_u = x.as_ptr() as *const u64;
        let y_u = y.as_ptr() as *const u64;
        while 8 * i < x.len() - 8 {
            let dist = (*x_u.add(i) ^ *y_u.add(i)).count_ones();
            accum += dist as u64;
            i += 1;
        }
    }

    i *= 8;

    if i < x.len() {
        accum += distance_naive(&x[i..], &y[i..]);
    }
    accum
}

pub fn distance(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return lib_avx2::distance_vect(x, y);
        }
        distance_faster(x, y)
    }
}

pub mod lib_avx2;

#[cfg(test)]
pub mod lib_test;
