#[cfg(feature = "0.5")]
use std::os::raw::*;

#[allow(non_snake_case)]
pub const fn WEBP_ABI_IS_INCOMPATIBLE(a: u16, b: u16) -> bool {
    (a >> 8) != (b >> 8)
}

#[cfg_attr(not(test), link(name = "webp"))]
extern "C" {
    #[cfg(feature = "1.1")]
    pub fn WebPMalloc(size: usize) -> *mut c_void;
    #[cfg(feature = "0.5")]
    pub fn WebPFree(ptr: *mut c_void);
}
