use std::os::raw::*;

#[cfg(feature = "0_5")]
use crate::encode::{WebPConfig, WebPPicture};
use crate::mux_types::*;

pub const WEBP_MUX_ABI_VERSION: c_int = WEBP_MUX_ABI_VERSION_INTERNAL;

cfg_if! {
    if #[cfg(feature = "1_4")] {
        const WEBP_MUX_ABI_VERSION_INTERNAL: c_int = 0x0109;
    } else if #[cfg(feature = "0_6")] {
        const WEBP_MUX_ABI_VERSION_INTERNAL: c_int = 0x0108;
    } else if #[cfg(feature = "0_5")] {
        const WEBP_MUX_ABI_VERSION_INTERNAL: c_int = 0x0106;
    } else {
        const WEBP_MUX_ABI_VERSION_INTERNAL: c_int = 0x0101;
    }
}

#[cfg(feature = "extern-types")]
#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "mux")))]
unsafe extern "C" {
    pub type WebPMux;
}

#[cfg(not(feature = "extern-types"))]
#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "mux")))]
#[repr(C)]
pub struct WebPMux(c_void);

#[allow(non_camel_case_types)]
// #[cfg_attr(feature = "must-use", must_use)] // meaningless for type aliases
pub type WebPMuxError = i32;

pub const WEBP_MUX_OK: WebPMuxError = 1;
pub const WEBP_MUX_NOT_FOUND: WebPMuxError = 0;
pub const WEBP_MUX_INVALID_ARGUMENT: WebPMuxError = -1;
pub const WEBP_MUX_BAD_DATA: WebPMuxError = -2;
pub const WEBP_MUX_MEMORY_ERROR: WebPMuxError = -3;
pub const WEBP_MUX_NOT_ENOUGH_DATA: WebPMuxError = -4;

#[allow(non_camel_case_types)]
pub type WebPChunkId = u32;

pub const WEBP_CHUNK_VP8X: WebPChunkId = 0;
pub const WEBP_CHUNK_ICCP: WebPChunkId = 1;
pub const WEBP_CHUNK_ANIM: WebPChunkId = 2;
pub const WEBP_CHUNK_ANMF: WebPChunkId = 3;
#[cfg(not(feature = "0_6"))]
#[deprecated(note = "Removed as of libwebp 0.6.0")]
pub const WEBP_CHUNK_FRGM: WebPChunkId = 4;
#[cfg(feature = "0_6")]
pub const WEBP_CHUNK_DEPRECATED: WebPChunkId = 4;
pub const WEBP_CHUNK_ALPHA: WebPChunkId = 5;
pub const WEBP_CHUNK_IMAGE: WebPChunkId = 6;
pub const WEBP_CHUNK_EXIF: WebPChunkId = 7;
pub const WEBP_CHUNK_XMP: WebPChunkId = 8;
pub const WEBP_CHUNK_UNKNOWN: WebPChunkId = 9;
pub const WEBP_CHUNK_NIL: WebPChunkId = 10;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: c_int,
    pub y_offset: c_int,
    pub duration: c_int,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    #[doc(hidden)]
    pub pad: [u32; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPMuxAnimParams {
    pub bgcolor: u32,
    pub loop_count: c_int,
}

#[cfg(all(feature = "0_5", feature = "extern-types"))]
#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "0_5")))]
extern "C" {
    pub type WebPAnimEncoder;
}

#[cfg(all(feature = "0_5", not(feature = "extern-types")))]
#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "0_5")))]
#[repr(C)]
pub struct WebPAnimEncoder(c_void);

#[cfg(feature = "0_5")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPAnimEncoderOptions {
    pub anim_params: WebPMuxAnimParams,
    pub minimize_size: c_int,
    pub kmin: c_int,
    pub kmax: c_int,
    pub allow_mixed: c_int,
    pub verbose: c_int,
    #[doc(hidden)]
    pub padding: [u32; 4],
}

unsafe extern "C" {
    pub fn WebPGetMuxVersion() -> c_int;
    #[doc(hidden)]
    #[cfg_attr(feature = "must-use", must_use)]
    pub fn WebPNewInternal(_: c_int) -> *mut WebPMux;
    pub fn WebPMuxDelete(mux: *mut WebPMux);
    #[doc(hidden)]
    #[cfg_attr(feature = "must-use", must_use)]
    pub fn WebPMuxCreateInternal(_: *const WebPData, _: c_int, _: c_int) -> *mut WebPMux;
    pub fn WebPMuxSetChunk(
        mux: *mut WebPMux,
        fourcc: *const c_char,
        chunk_data: *const WebPData,
        copy_data: c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxGetChunk(
        mux: *const WebPMux,
        fourcc: *const c_char,
        chunk_data: *mut WebPData,
    ) -> WebPMuxError;
    pub fn WebPMuxDeleteChunk(mux: *mut WebPMux, fourcc: *const c_char) -> WebPMuxError;
    pub fn WebPMuxSetImage(
        mux: *mut WebPMux,
        bitstream: *const WebPData,
        copy_data: c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxPushFrame(
        mux: *mut WebPMux,
        frame: *const WebPMuxFrameInfo,
        copy_data: c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxGetFrame(
        mux: *const WebPMux,
        nth: u32,
        frame: *mut WebPMuxFrameInfo,
    ) -> WebPMuxError;
    pub fn WebPMuxDeleteFrame(mux: *mut WebPMux, nth: u32) -> WebPMuxError;
    pub fn WebPMuxSetAnimationParams(
        mux: *mut WebPMux,
        params: *const WebPMuxAnimParams,
    ) -> WebPMuxError;
    pub fn WebPMuxGetAnimationParams(
        mux: *const WebPMux,
        params: *mut WebPMuxAnimParams,
    ) -> WebPMuxError;
    #[cfg(feature = "0_5")]
    pub fn WebPMuxSetCanvasSize(mux: *mut WebPMux, width: c_int, height: c_int) -> WebPMuxError;
    pub fn WebPMuxGetCanvasSize(
        mux: *const WebPMux,
        width: *mut c_int,
        height: *mut c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxGetFeatures(mux: *const WebPMux, flags: *mut u32) -> WebPMuxError;
    pub fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut c_int,
    ) -> WebPMuxError;
    pub fn WebPMuxAssemble(mux: *mut WebPMux, assembled_data: *mut WebPData) -> WebPMuxError;
    #[cfg(feature = "0_5")]
    #[doc(hidden)]
    pub fn WebPAnimEncoderOptionsInitInternal(_: *mut WebPAnimEncoderOptions, _: c_int) -> c_int;
    #[cfg(feature = "0_5")]
    #[doc(hidden)]
    pub fn WebPAnimEncoderNewInternal(
        _: c_int,
        _: c_int,
        _: *const WebPAnimEncoderOptions,
        _: c_int,
    ) -> *mut WebPAnimEncoder;
    #[cfg(feature = "0_5")]
    #[cfg_attr(feature = "must-use", must_use)]
    pub fn WebPAnimEncoderAdd(
        enc: *mut WebPAnimEncoder,
        frame: *mut WebPPicture,
        timestamp_ms: c_int,
        config: *const WebPConfig,
    ) -> c_int;
    #[cfg(feature = "0_5")]
    #[cfg_attr(feature = "must-use", must_use)]
    pub fn WebPAnimEncoderAssemble(enc: *mut WebPAnimEncoder, webp_data: *mut WebPData) -> c_int;
    #[cfg(feature = "0_5")]
    pub fn WebPAnimEncoderGetError(enc: *mut WebPAnimEncoder) -> *const c_char;
    #[cfg(feature = "0_5")]
    pub fn WebPAnimEncoderDelete(enc: *mut WebPAnimEncoder);
    #[cfg(feature = "1_4")]
    pub fn WebPAnimEncoderSetChunk(
        enc: *mut WebPAnimEncoder,
        fourcc: *const c_char,
        chunk_data: *const WebPData,
        copy_data: c_int,
    ) -> WebPMuxError;
    #[cfg(feature = "1_4")]
    pub fn WebPAnimEncoderGetChunk(
        enc: *const WebPAnimEncoder,
        fourcc: *const c_char,
        chunk_data: *mut WebPData,
    ) -> WebPMuxError;
    #[cfg(feature = "1_4")]
    pub fn WebPAnimEncoderDeleteChunk(
        enc: *mut WebPAnimEncoder,
        fourcc: *const c_char,
    ) -> WebPMuxError;
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "must-use", must_use)]
#[inline]
pub unsafe extern "C" fn WebPMuxNew() -> *mut WebPMux {
    unsafe { WebPNewInternal(WEBP_MUX_ABI_VERSION) }
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "must-use", must_use)]
#[inline]
pub unsafe extern "C" fn WebPMuxCreate(
    bitstream: *const WebPData,
    copy_data: c_int,
) -> *mut WebPMux {
    unsafe { WebPMuxCreateInternal(bitstream, copy_data, WEBP_MUX_ABI_VERSION) }
}

#[cfg(feature = "0_5")]
#[allow(non_snake_case)]
#[cfg_attr(feature = "must-use", must_use)]
#[inline]
pub unsafe extern "C" fn WebPAnimEncoderOptionsInit(
    enc_options: *mut WebPAnimEncoderOptions,
) -> c_int {
    unsafe { WebPAnimEncoderOptionsInitInternal(enc_options, WEBP_MUX_ABI_VERSION) }
}

#[cfg(feature = "0_5")]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPAnimEncoderNew(
    width: c_int,
    height: c_int,
    enc_options: *const WebPAnimEncoderOptions,
) -> *mut WebPAnimEncoder {
    unsafe { WebPAnimEncoderNewInternal(width, height, enc_options, WEBP_MUX_ABI_VERSION) }
}
