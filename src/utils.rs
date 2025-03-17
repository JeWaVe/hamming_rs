// TODO: arch checks and windows version
extern "C" {
    #[cfg(target_os = "linux")]
    pub fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    #[cfg(target_os = "windows")]
    pub fn _aligned_malloc(size: usize, alignment: usize) -> *mut u8;
}

#[cfg(target_os = "linux")]
pub fn aligned_malloc(alignment: usize, size: usize) -> *mut u8 {
    unsafe {
        return aligned_alloc(alignment, size);
    }
}

#[cfg(target_os = "windows")]
pub fn aligned_malloc(alignment: usize, size: usize) -> *mut u8 {
    unsafe {
        return _aligned_malloc(size, alignment);
    }
}
