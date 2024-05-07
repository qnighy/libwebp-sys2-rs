use std::os::raw::*;
use std::ptr;

#[cfg(feature = "0_5")]
use crate::decode::*;
use crate::mux_types::*;

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
pub const WEBP_DEMUX_ABI_VERSION: c_int = WEBP_DEMUX_ABI_VERSION_INTERNAL;

cfg_if! {
    if #[cfg(feature = "0_5")] {
        const WEBP_DEMUX_ABI_VERSION_INTERNAL: c_int = 0x0107;
    } else {
        const WEBP_DEMUX_ABI_VERSION_INTERNAL: c_int = 0x0101;
    }
}

#[cfg(feature = "extern-types")]
extern "C" {
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub type WebPDemuxer;
}

#[cfg(not(feature = "extern-types"))]
#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[repr(C)]
pub struct WebPDemuxer(c_void);

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[allow(non_camel_case_types)]
pub type WebPDemuxState = i32;

pub const WEBP_DEMUX_PARSE_ERROR: WebPDemuxState = -1;
pub const WEBP_DEMUX_PARSING_HEADER: WebPDemuxState = 0;
pub const WEBP_DEMUX_PARSED_HEADER: WebPDemuxState = 1;
pub const WEBP_DEMUX_DONE: WebPDemuxState = 2;

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[allow(non_camel_case_types)]
pub type WebPFormatFeature = u32;

pub const WEBP_FF_FORMAT_FLAGS: WebPFormatFeature = 0;
pub const WEBP_FF_CANVAS_WIDTH: WebPFormatFeature = 1;
pub const WEBP_FF_CANVAS_HEIGHT: WebPFormatFeature = 2;
pub const WEBP_FF_LOOP_COUNT: WebPFormatFeature = 3;
pub const WEBP_FF_BACKGROUND_COLOR: WebPFormatFeature = 4;
pub const WEBP_FF_FRAME_COUNT: WebPFormatFeature = 5;

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPIterator {
    pub frame_num: c_int,
    pub num_frames: c_int,
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub fragment_num: c_int,
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub num_fragments: c_int,
    pub x_offset: c_int,
    pub y_offset: c_int,
    pub width: c_int,
    pub height: c_int,
    pub duration: c_int,
    pub dispose_method: WebPMuxAnimDispose,
    pub complete: c_int,
    pub fragment: WebPData,
    pub has_alpha: c_int,
    pub blend_method: WebPMuxAnimBlend,
    #[doc(hidden)]
    pub pad: [u32; 2],
    #[doc(hidden)]
    pub private_: *mut c_void,
}

unsafe impl Send for WebPIterator {}

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPChunkIterator {
    pub chunk_num: c_int,
    pub num_chunks: c_int,
    pub chunk: WebPData,
    #[doc(hidden)]
    pub pad: [u32; 6],
    #[doc(hidden)]
    pub private_: *mut c_void,
}

unsafe impl Send for WebPChunkIterator {}

#[cfg(all(feature = "0_5", feature = "extern-types"))]
extern "C" {
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub type WebPAnimDecoder;
}

#[cfg(all(feature = "0_5", not(feature = "extern-types")))]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0_5")))
)]
#[repr(C)]
pub struct WebPAnimDecoder(c_void);

#[cfg(feature = "0_5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0_5")))
)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPAnimDecoderOptions {
    pub color_mode: WEBP_CSP_MODE,
    pub use_threads: c_int,
    #[doc(hidden)]
    pub padding: [u32; 7],
}

#[cfg(feature = "0_5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0_5")))
)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPAnimInfo {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub loop_count: u32,
    pub bgcolor: u32,
    pub frame_count: u32,
    #[doc(hidden)]
    pub pad: [u32; 4],
}

extern "C" {
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPGetDemuxVersion() -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    #[doc(hidden)]
    pub fn WebPDemuxInternal(
        _: *const WebPData,
        _: c_int,
        _: *mut WebPDemuxState,
        _: c_int,
    ) -> *mut WebPDemuxer;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxDelete(dmux: *mut WebPDemuxer);
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxGetI(dmux: *const WebPDemuxer, feature: WebPFormatFeature) -> u32;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxGetFrame(
        dmux: *const WebPDemuxer,
        frame_number: c_int,
        iter: *mut WebPIterator,
    ) -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxNextFrame(iter: *mut WebPIterator) -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxPrevFrame(iter: *mut WebPIterator) -> c_int;
    #[cfg(not(feature = "0_5"))]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub fn WebPDemuxSelectFragment(iter: *mut WebPIterator, fragment_num: c_int) -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxReleaseIterator(iter: *mut WebPIterator);
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxGetChunk(
        dmux: *const WebPDemuxer,
        fourcc: *const c_char,
        chunk_number: c_int,
        iter: *mut WebPChunkIterator,
    ) -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxNextChunk(iter: *mut WebPChunkIterator) -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxPrevChunk(iter: *mut WebPChunkIterator) -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPDemuxReleaseChunkIterator(iter: *mut WebPChunkIterator);
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    #[doc(hidden)]
    pub fn WebPAnimDecoderOptionsInitInternal(_: *mut WebPAnimDecoderOptions, _: c_int) -> c_int;
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    #[doc(hidden)]
    pub fn WebPAnimDecoderNewInternal(
        _: *const WebPData,
        _: *const WebPAnimDecoderOptions,
        _: c_int,
    ) -> *mut WebPAnimDecoder;
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub fn WebPAnimDecoderGetInfo(dec: *const WebPAnimDecoder, info: *mut WebPAnimInfo) -> c_int;
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub fn WebPAnimDecoderGetNext(
        dec: *mut WebPAnimDecoder,
        buf: *mut *mut u8,
        timestamp: *mut c_int,
    ) -> c_int;
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub fn WebPAnimDecoderHasMoreFrames(dec: *const WebPAnimDecoder) -> c_int;
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub fn WebPAnimDecoderReset(dec: *mut WebPAnimDecoder);
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub fn WebPAnimDecoderGetDemuxer(dec: *const WebPAnimDecoder) -> *const WebPDemuxer;
    #[cfg(feature = "0_5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0_5")))
    )]
    pub fn WebPAnimDecoderDelete(dec: *mut WebPAnimDecoder);
}

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPDemux(data: *const WebPData) -> *mut WebPDemuxer {
    WebPDemuxInternal(data, 0, ptr::null_mut(), WEBP_DEMUX_ABI_VERSION)
}

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPDemuxPartial(
    data: *const WebPData,
    state: *mut WebPDemuxState,
) -> *mut WebPDemuxer {
    WebPDemuxInternal(data, 1, state, WEBP_DEMUX_ABI_VERSION)
}

#[cfg(feature = "0_5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0_5")))
)]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPAnimDecoderOptionsInit(
    dec_options: *mut WebPAnimDecoderOptions,
) -> c_int {
    WebPAnimDecoderOptionsInitInternal(dec_options, WEBP_DEMUX_ABI_VERSION)
}

#[cfg(feature = "0_5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0_5")))
)]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPAnimDecoderNew(
    webp_data: *const WebPData,
    dec_options: *const WebPAnimDecoderOptions,
) -> *mut WebPAnimDecoder {
    WebPAnimDecoderNewInternal(webp_data, dec_options, WEBP_DEMUX_ABI_VERSION)
}
