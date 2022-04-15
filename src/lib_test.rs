fn random_vector(x: &mut [u8]) {
    for i in 0..x.len() {
        x[i] = rand::random()
    }
}

#[test]
pub fn test_naive() {
    let mut x = [0; 12];
    let mut y = [0; 12];
    random_vector(&mut x);
    random_vector(&mut y);
    let a = super::distance_naive(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_faster() {
    unsafe {
        let mut x = [0; 12];
        let mut y = [0; 12];
        random_vector(&mut x);
        random_vector(&mut y);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_faster_2() {
    unsafe {
        let mut x = [0; 16];
        let mut y = [0; 16];
        random_vector(&mut x);
        random_vector(&mut y);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_faster_3() {
    unsafe {
        let mut x = [0; 4];
        let mut y = [0; 4];
        random_vector(&mut x);
        random_vector(&mut y);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_faster_4() {
    unsafe {
        let mut x = [0; 32];
        let mut y = [0; 32];
        random_vector(&mut x);
        random_vector(&mut y);
        let a = super::distance_faster(&x, &y);
        let b = hamming::distance(&x, &y);
        assert_eq!(a, b);
    }
}

#[test]
pub fn test_avx() {
    let mut x = [0; 1024];
    let mut y = [0; 1024];
    random_vector(&mut x);
    random_vector(&mut y);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_avx_2() {
    let mut x = [0; 12];
    let mut y = [0; 12];
    random_vector(&mut x);
    random_vector(&mut y);
    let a = super::distance(&x, &y);
    let b = hamming::distance(&x, &y);
    assert_eq!(a, b);
}

#[test]
pub fn test_avx_3() {
    let mut x = [0; 128];
    let mut y = [0; 128];
    random_vector(&mut x);
    random_vector(&mut y);
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
