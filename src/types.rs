#[cfg(feature = "0_5")]
use std::os::raw::*;

/// Macro to check ABI compatibility (same major revision number)
#[allow(non_snake_case)]
#[inline]
pub const fn WEBP_ABI_IS_INCOMPATIBLE(a: u16, b: u16) -> bool {
    (a >> 8) != (b >> 8)
}

unsafe extern "C" {
    /// Allocates `size` bytes of memory. Returns NULL upon error. Memory
    /// must be deallocated by calling `WebPFree()`. This function is made available
    /// by the core `libwebp` library.
    #[cfg(feature = "1_1")]
    #[must_use]
    pub fn WebPMalloc(size: usize) -> *mut c_void;
    /// Releases memory returned by the `WebPDecode*()` functions (from `decode.h`).
    #[cfg(feature = "0_5")]
    pub fn WebPFree(ptr: *mut c_void);
}
