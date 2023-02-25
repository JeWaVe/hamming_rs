// TODO: arch checks and windows version
extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "aligned_alloc"]
    fn _aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    #[cfg(target_os = "windows")]
    fn _aligned_malloc(size: usize, alignment: usize) -> *mut u8;
}

pub fn aligned_alloc(alignment: usize, size: usize) -> *mut u8 {
    unsafe {
        #[cfg(target_os = "linux")]
        return _aligned_alloc(alignment, size);

        #[cfg(target_os = "windows")]
        return _aligned_malloc(size, alignment);
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    compile_error!("unsupported os");
}
