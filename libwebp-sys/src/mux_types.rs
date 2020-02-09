use std::mem;
use std::os::raw::*;

use libc::c_void as libc_void;
use libc::{free, malloc, memcpy, memset};

pub use self::WebPFeatureFlags::*;
pub use self::WebPMuxAnimBlend::*;
pub use self::WebPMuxAnimDispose::*;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPFeatureFlags {
    #[cfg(not(feature = "0.6.0"))]
    FRAGMENTS_FLAG = 0x00000001,
    ANIMATION_FLAG = 0x00000002,
    XMP_FLAG = 0x00000004,
    EXIF_FLAG = 0x00000008,
    ALPHA_FLAG = 0x00000010,
    ICCP_FLAG = 0x00000020,
    #[cfg(feature = "0.6.0")]
    ALL_VALID_FLAGS = 0x0000003E,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPMuxAnimDispose {
    WEBP_MUX_DISPOSE_NONE = 0,
    WEBP_MUX_DISPOSE_BACKGROUND = 1,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPMuxAnimBlend {
    WEBP_MUX_BLEND = 0,
    WEBP_MUX_NO_BLEND = 1,
}

#[repr(C)]
pub struct WebPData {
    pub bytes: *const u8,
    pub size: usize,
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPDataInit(webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        memset(webp_data as *mut libc_void, 0, mem::size_of::<WebPData>());
    }
}

// Clears the contents of the 'webp_data' object by calling free(). Does not
// deallocate the object itself.
#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPDataClear(webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        free((*webp_data).bytes as *mut libc_void);
        WebPDataInit(webp_data);
    }
}

// Allocates necessary storage for 'dst' and copies the contents of 'src'.
// Returns true on success.
#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPDataCopy(src: *const WebPData, dst: *mut WebPData) -> c_int {
    if src.is_null() || dst.is_null() {
        return 0;
    }
    WebPDataInit(dst);
    if !(*src).bytes.is_null() && (*src).size != 0 {
        (*dst).bytes = malloc((*src).size) as *mut u8;
        if (*dst).bytes.is_null() {
            return 0;
        }
        memcpy(
            (*dst).bytes as *mut libc_void,
            (*src).bytes as *const libc_void,
            (*src).size,
        );
        (*dst).size = (*src).size;
    }
    return 1;
}
