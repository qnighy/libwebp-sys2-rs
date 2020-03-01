#[cfg(feature = "0_5")]
use std::os::raw::*;

/// Macro to check ABI compatibility (same major revision number)
#[allow(non_snake_case)]
#[inline]
pub const fn WEBP_ABI_IS_INCOMPATIBLE(a: u16, b: u16) -> bool {
    (a >> 8) != (b >> 8)
}

extern "C" {
    /// Allocates `size` bytes of memory. Returns NULL upon error. Memory
    /// must be deallocated by calling `WebPFree()`. This function is made available
    /// by the core `libwebp` library.
    #[cfg(feature = "1_1")]
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "1_1")))]
    pub fn WebPMalloc(size: usize) -> *mut c_void;
    /// Releases memory returned by the `WebPDecode*()` functions (from `decode.h`).
    #[cfg(feature = "0_5")]
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "0_5")))]
    pub fn WebPFree(ptr: *mut c_void);
}

#[cfg(all(test, feature = "1_1"))]
mod tests {
    use super::*;

    #[cfg(feature = "1_1")]
    #[test]
    fn test_malloc() {
        unsafe {
            let ptr = WebPMalloc(12);
            assert!(!ptr.is_null());
            WebPFree(ptr);
        }
    }
}
