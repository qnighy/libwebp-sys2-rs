use std::mem;
use std::os::raw::*;

use libc::{memcpy, memset};

#[cfg(feature = "1.1")]
use crate::{WebPFree, WebPMalloc};
#[cfg(not(feature = "1.1"))]
use libc::{free as WebPFree, malloc as WebPMalloc};

#[allow(non_camel_case_types)]
pub type WebPFeatureFlags = u32;

#[cfg(not(feature = "0.6.0"))]
pub const FRAGMENTS_FLAG: WebPFeatureFlags = 0x00000001;
pub const ANIMATION_FLAG: WebPFeatureFlags = 0x00000002;
pub const XMP_FLAG: WebPFeatureFlags = 0x00000004;
pub const EXIF_FLAG: WebPFeatureFlags = 0x00000008;
pub const ALPHA_FLAG: WebPFeatureFlags = 0x00000010;
pub const ICCP_FLAG: WebPFeatureFlags = 0x00000020;
#[cfg(feature = "0.6.0")]
pub const ALL_VALID_FLAGS: WebPFeatureFlags = 0x0000003E;

#[allow(non_camel_case_types)]
pub type WebPMuxAnimDispose = u32;

pub const WEBP_MUX_DISPOSE_NONE: WebPMuxAnimDispose = 0;
pub const WEBP_MUX_DISPOSE_BACKGROUND: WebPMuxAnimDispose = 1;

#[allow(non_camel_case_types)]
pub type WebPMuxAnimBlend = u32;

pub const WEBP_MUX_BLEND: WebPMuxAnimBlend = 0;
pub const WEBP_MUX_NO_BLEND: WebPMuxAnimBlend = 1;

#[repr(C)]
pub struct WebPData {
    pub bytes: *const u8,
    pub size: usize,
}

#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPDataInit(webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        memset(webp_data as *mut c_void, 0, mem::size_of::<WebPData>());
    }
}

// Clears the contents of the 'webp_data' object by calling free(). Does not
// deallocate the object itself.
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPDataClear(webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        WebPFree((*webp_data).bytes as *mut c_void);
        WebPDataInit(webp_data);
    }
}

// Allocates necessary storage for 'dst' and copies the contents of 'src'.
// Returns true on success.
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPDataCopy(src: *const WebPData, dst: *mut WebPData) -> c_int {
    if src.is_null() || dst.is_null() {
        return 0;
    }
    WebPDataInit(dst);
    if !(*src).bytes.is_null() && (*src).size != 0 {
        (*dst).bytes = WebPMalloc((*src).size) as *mut u8;
        if (*dst).bytes.is_null() {
            return 0;
        }
        memcpy(
            (*dst).bytes as *mut c_void,
            (*src).bytes as *const c_void,
            (*src).size,
        );
        (*dst).size = (*src).size;
    }
    return 1;
}
