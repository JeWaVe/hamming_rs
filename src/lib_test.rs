unsafe fn random_data(x: *mut u8, len: usize) {
    for i in 0..len {
        *x.add(i) = rand::random();
    }
}

// TODO: arch checks and windows version
extern "C" {
    fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
}

fn random_slice<'a>(size: usize) -> &'a [u8] {
    unsafe {
        let ptr = aligned_alloc(64, size);
        random_data(ptr, size);
        return std::slice::from_raw_parts(ptr, size);
    }
}

#[test]
pub fn test_naive() {
    let x = random_slice(12);
    let y = random_slice(12);
    let a = super::distance_naive(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_faster() {
    unsafe {
        let x = random_slice(12);
        let y = random_slice(12);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_faster_2() {
    unsafe {
        let x = random_slice(16);
        let y = random_slice(16);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_faster_3() {
    unsafe {
        let x = random_slice(4);
        let y = random_slice(4);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_faster_4() {
    unsafe {
        let x = random_slice(32);
        let y = random_slice(32);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_avx() {
    let x = random_slice(1024);
    let y = random_slice(1024);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_avx_2() {
    let x = random_slice(12);
    let y = random_slice(12);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_avx_3() {
    let x = random_slice(128);
    let y = random_slice(128);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
