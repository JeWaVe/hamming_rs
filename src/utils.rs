// TODO: arch checks and windows version
extern "C" {
    #[cfg(target_os = "linux")]
    fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    #[cfg(target_os = "windows")]
    fn _aligned_malloc(size: usize, alignment: usize) -> *mut u8;
}

pub fn aligned_alloc(alignment: usize, size: usize) -> *mut u8 {
    unsafe {
        if cfg!(linux) {
            return aligned_alloc(alignment, size);
        }
        if cfg!(windows) {
            return _aligned_malloc(size, alignment);
        }
    }

    panic!("unsupported os");
}
