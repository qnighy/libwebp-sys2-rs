use std::os::raw::*;
use std::ptr;

#[cfg(feature = "0.5")]
use crate::decode::*;
use crate::mux_types::*;

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
pub const WEBP_DEMUX_ABI_VERSION: c_int = WEBP_DEMUX_ABI_VERSION_INTERNAL;

cfg_if! {
    if #[cfg(feature = "0.5")] {
        const WEBP_DEMUX_ABI_VERSION_INTERNAL: c_int = 0x0107;
    } else {
        const WEBP_DEMUX_ABI_VERSION_INTERNAL: c_int = 0x0101;
    }
}
// extern {
//     type WebPDemuxer;
// }
#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[repr(C)]
pub struct WebPDemuxer(c_void);

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPDemuxState {
    WEBP_DEMUX_PARSE_ERROR = -1,
    WEBP_DEMUX_PARSING_HEADER = 0,
    WEBP_DEMUX_PARSED_HEADER = 1,
    WEBP_DEMUX_DONE = 2,
}

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPFormatFeature {
    WEBP_FF_FORMAT_FLAGS = 0,
    WEBP_FF_CANVAS_WIDTH = 1,
    WEBP_FF_CANVAS_HEIGHT = 2,
    WEBP_FF_LOOP_COUNT = 3,
    WEBP_FF_BACKGROUND_COLOR = 4,
    WEBP_FF_FRAME_COUNT = 5,
}

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[repr(C)]
pub struct WebPIterator {
    pub frame_num: c_int,
    pub num_frames: c_int,
    #[cfg(not(feature = "0.5"))]
    pub fragment_num: c_int,
    #[cfg(not(feature = "0.5"))]
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
    pub pad: [u32; 2],
    private_: *mut c_void,
}

#[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
#[repr(C)]
pub struct WebPChunkIterator {
    pub chunk_num: c_int,
    pub num_chunks: c_int,
    pub chunk: WebPData,
    pub pad: [u32; 6],
    private_: *mut c_void,
}

// extern {
//     #[cfg(feature = "0.5")]
//     pub type WebPAnimDecoder;
// }
#[cfg(feature = "0.5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0.5")))
)]
#[repr(C)]
pub struct WebPAnimDecoder(c_void);

#[cfg(feature = "0.5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0.5")))
)]
#[repr(C)]
pub struct WebPAnimDecoderOptions {
    pub color_mode: WEBP_CSP_MODE,
    pub use_threads: c_int,
    pub padding: [u32; 7],
}

#[cfg(feature = "0.5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0.5")))
)]
#[repr(C)]
pub struct WebPAnimInfo {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub loop_count: u32,
    pub bgcolor: u32,
    pub frame_count: u32,
    pub pad: [u32; 4],
}

extern "C" {
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
    pub fn WebPGetDemuxVersion() -> c_int;
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "demux")))]
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
    #[cfg(not(feature = "0.5"))]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
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
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderOptionsInitInternal(_: *mut WebPAnimDecoderOptions, _: c_int) -> c_int;
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderNewInternal(
        _: *const WebPData,
        _: *const WebPAnimDecoderOptions,
        _: c_int,
    ) -> *mut WebPAnimDecoder;
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderGetInfo(dec: *const WebPAnimDecoder, info: *mut WebPAnimInfo) -> c_int;
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderGetNext(
        dec: *mut WebPAnimDecoder,
        buf: *mut *mut u8,
        timestamp: *mut c_int,
    ) -> c_int;
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderHasMoreFrames(dec: *const WebPAnimDecoder) -> c_int;
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderReset(dec: *mut WebPAnimDecoder);
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
    )]
    pub fn WebPAnimDecoderGetDemuxer(dec: *const WebPAnimDecoder) -> *const WebPDemuxer;
    #[cfg(feature = "0.5")]
    #[cfg_attr(
        feature = "__doc_cfg",
        doc(cfg(all(feature = "demux", feature = "0.5")))
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

#[cfg(feature = "0.5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0.5")))
)]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPAnimDecoderOptionsInit(
    dec_options: *mut WebPAnimDecoderOptions,
) -> c_int {
    WebPAnimDecoderOptionsInitInternal(dec_options, WEBP_DEMUX_ABI_VERSION)
}

#[cfg(feature = "0.5")]
#[cfg_attr(
    feature = "__doc_cfg",
    doc(cfg(all(feature = "demux", feature = "0.5")))
)]
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPAnimDecoderNew(
    webp_data: *const WebPData,
    dec_options: *const WebPAnimDecoderOptions,
) -> *mut WebPAnimDecoder {
    WebPAnimDecoderNewInternal(webp_data, dec_options, WEBP_DEMUX_ABI_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEBP_IMAGE: [u8; 94] = [
        0x52, 0x49, 0x46, 0x46, 0x56, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38,
        0x20, 0x4A, 0x00, 0x00, 0x00, 0xD0, 0x01, 0x00, 0x9D, 0x01, 0x2A, 0x03, 0x00, 0x02, 0x00,
        0x02, 0x00, 0x34, 0x25, 0xA8, 0x02, 0x74, 0x01, 0x0E, 0xFE, 0x03, 0x8E, 0x00, 0x00, 0xFE,
        0xAD, 0xFF, 0xF1, 0x5C, 0xB4, 0xF8, 0xED, 0xFF, 0xF0, 0xC0, 0xBA, 0xBF, 0x93, 0x05, 0xEA,
        0x0C, 0x9F, 0x93, 0x3F, 0xE8, 0xC0, 0xBF, 0x3F, 0xFF, 0xA9, 0xBF, 0xFF, 0x24, 0x7B, 0xCB,
        0xFF, 0x46, 0x05, 0xF9, 0xFF, 0xFD, 0x4D, 0xFE, 0x30, 0xE5, 0x86, 0xAA, 0x07, 0x31, 0x23,
        0x6F, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn test_new_and_delete() {
        unsafe {
            let data = WebPData {
                bytes: WEBP_IMAGE.as_ptr(),
                size: WEBP_IMAGE.len(),
            };
            let ptr = WebPDemux(&data);
            assert!(!ptr.is_null());
            WebPDemuxDelete(ptr);
        }
    }
}
