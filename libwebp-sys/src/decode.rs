use std::os::raw::*;
use std::ptr;

pub use self::VP8StatusCode::*;
pub use self::WEBP_CSP_MODE::*;

cfg_if! {
    if #[cfg(feature = "0.5")] {
        pub const WEBP_DECODER_ABI_VERSION: c_int = 0x0208;
    } else {
        pub const WEBP_DECODER_ABI_VERSION: c_int = 0x0203;
    }
}

// extern {
//     type WebPIDecoder;
// }
#[repr(C)]
pub struct WebPIDecoder(c_void);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum WEBP_CSP_MODE {
    MODE_RGB = 0,
    MODE_RGBA = 1,
    MODE_BGR = 2,
    MODE_BGRA = 3,
    MODE_ARGB = 4,
    MODE_RGBA_4444 = 5,
    MODE_RGB_565 = 6,
    MODE_rgbA = 7,
    MODE_bgrA = 8,
    MODE_Argb = 9,
    MODE_rgbA_4444 = 10,
    MODE_YUV = 11,
    MODE_YUVA = 12,
}

impl WEBP_CSP_MODE {
    pub const MODE_LAST: c_int = 13;
}

pub const MODE_LAST: c_int = WEBP_CSP_MODE::MODE_LAST;

#[allow(non_snake_case)]
pub extern "C" fn WebPIsPremultipliedMode(mode: WEBP_CSP_MODE) -> c_int {
    (mode == MODE_rgbA || mode == MODE_bgrA || mode == MODE_Argb || mode == MODE_rgbA_4444) as c_int
}

#[allow(non_snake_case)]
pub extern "C" fn WebPIsAlphaMode(mode: WEBP_CSP_MODE) -> c_int {
    (mode == MODE_RGBA
        || mode == MODE_BGRA
        || mode == MODE_ARGB
        || mode == MODE_RGBA_4444
        || mode == MODE_YUVA
        || WebPIsPremultipliedMode(mode) != 0) as c_int
}

#[allow(non_snake_case)]
pub extern "C" fn WebPIsRGBMode(mode: WEBP_CSP_MODE) -> c_int {
    (mode < MODE_YUV) as c_int
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WebPRGBABuffer {
    pub rgba: *mut u8,
    pub stride: c_int,
    pub size: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WebPYUVABuffer {
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub a: *mut u8,
    pub y_stride: c_int,
    pub u_stride: c_int,
    pub v_stride: c_int,
    pub a_stride: c_int,
    pub y_size: usize,
    pub u_size: usize,
    pub v_size: usize,
    pub a_size: usize,
}

#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: c_int,
    pub height: c_int,
    pub is_external_memory: c_int,
    pub u: WebPDecBufferUnion,
    pub pad: [u32; 4],
    pub private_memory: *mut u8,
}

#[allow(non_snake_case)]
#[repr(C)]
pub union WebPDecBufferUnion {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum VP8StatusCode {
    VP8_STATUS_OK = 0,
    VP8_STATUS_OUT_OF_MEMORY = 1,
    VP8_STATUS_INVALID_PARAM = 2,
    VP8_STATUS_BITSTREAM_ERROR = 3,
    VP8_STATUS_UNSUPPORTED_FEATURE = 4,
    VP8_STATUS_SUSPENDED = 5,
    VP8_STATUS_USER_ABORT = 6,
    VP8_STATUS_NOT_ENOUGH_DATA = 7,
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPIDecGetYUV(
    idec: *const WebPIDecoder,
    last_y: *mut c_int,
    u: *mut *mut u8,
    v: *mut *mut u8,
    width: *mut c_int,
    height: *mut c_int,
    stride: *mut c_int,
    uv_stride: *mut c_int,
) -> *mut u8 {
    WebPIDecGetYUVA(
        idec,
        last_y,
        u,
        v,
        ptr::null_mut(),
        width,
        height,
        stride,
        uv_stride,
        ptr::null_mut(),
    )
}

#[repr(C)]
pub struct WebPBitstreamFeatures {
    pub width: c_int,
    pub height: c_int,
    pub has_alpha: c_int,
    pub has_animation: c_int,
    pub format: c_int,
    #[cfg(not(feature = "0.5"))]
    pub no_incremental_decoding: c_int,
    #[cfg(not(feature = "0.5"))]
    pub rotate: c_int,
    #[cfg(not(feature = "0.5"))]
    pub uv_sampling: c_int,
    #[cfg(not(feature = "0.5"))]
    pub pad: [u32; 2],
    #[cfg(feature = "0.5")]
    pub pad: [u32; 5],
}

#[repr(C)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: c_int,
    pub no_fancy_upsampling: c_int,
    pub use_cropping: c_int,
    pub crop_left: c_int,
    pub crop_top: c_int,
    pub crop_width: c_int,
    pub crop_height: c_int,
    pub use_scaling: c_int,
    pub scaled_width: c_int,
    pub scaled_height: c_int,
    pub use_threads: c_int,
    pub dithering_strength: c_int,
    #[cfg(feature = "0.5")]
    pub flip: c_int,
    #[cfg(feature = "0.5")]
    pub alpha_dithering_strength: c_int,
    #[cfg(not(feature = "0.5"))]
    pub force_rotation: c_int,
    #[cfg(not(feature = "0.5"))]
    pub no_enhancement: c_int,
    pub pad: [u32; 5],
}

#[repr(C)]
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}

#[link(name = "webp")]
extern "C" {
    pub fn WebPGetDecoderVersion() -> c_int;
    pub fn WebPGetInfo(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> c_int;
    pub fn WebPDecodeRGBA(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    pub fn WebPDecodeARGB(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    pub fn WebPDecodeBGRA(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    pub fn WebPDecodeRGB(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    pub fn WebPDecodeBGR(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    pub fn WebPDecodeYUV(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
        u: *mut *mut u8,
        v: *mut *mut u8,
        stride: *mut c_int,
        uv_stride: *mut c_int,
    ) -> *mut u8;
    #[cfg(feature = "0.5")]
    pub fn WebPFree(ptr: *mut c_void);
    pub fn WebPDecodeRGBAInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut u8;
    pub fn WebPDecodeARGBInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut u8;
    pub fn WebPDecodeBGRAInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut u8;
    pub fn WebPDecodeRGBInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut u8;
    pub fn WebPDecodeBGRInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut u8;
    pub fn WebPDecodeYUVInto(
        data: *const u8,
        data_size: usize,
        luma: *mut u8,
        luma_size: usize,
        luma_stride: c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: c_int,
    ) -> *mut u8;
    fn WebPInitDecBufferInternal(_: *mut WebPDecBuffer, _: c_int) -> c_int;
    pub fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
    pub fn WebPINewDecoder(output_buffer: *mut WebPDecBuffer) -> *mut WebPIDecoder;
    pub fn WebPINewRGB(
        csp: WEBP_CSP_MODE,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut WebPIDecoder;
    pub fn WebPINewYUVA(
        luma: *mut u8,
        luma_size: usize,
        luma_stride: c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: c_int,
        a: *mut u8,
        a_size: usize,
        a_stride: c_int,
    ) -> *mut WebPIDecoder;
    pub fn WebPINewYUV(
        luma: *mut u8,
        luma_size: usize,
        luma_stride: c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: c_int,
    ) -> *mut WebPIDecoder;
    pub fn WebPIDelete(idec: *mut WebPIDecoder);
    pub fn WebPIAppend(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
    pub fn WebPIUpdate(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
    pub fn WebPIDecGetRGB(
        idec: *const WebPIDecoder,
        last_y: *mut c_int,
        width: *mut c_int,
        height: *mut c_int,
        stride: *mut c_int,
    ) -> *mut u8;
    pub fn WebPIDecGetYUVA(
        idec: *const WebPIDecoder,
        last_y: *mut c_int,
        u: *mut *mut u8,
        v: *mut *mut u8,
        a: *mut *mut u8,
        width: *mut c_int,
        height: *mut c_int,
        stride: *mut c_int,
        uv_stride: *mut c_int,
        a_stride: *mut c_int,
    ) -> *mut u8;
    pub fn WebPIDecodedArea(
        idec: *const WebPIDecoder,
        left: *mut c_int,
        top: *mut c_int,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *const WebPDecBuffer;
    fn WebPGetFeaturesInternal(
        _: *const u8,
        _: usize,
        _: *mut WebPBitstreamFeatures,
        _: c_int,
    ) -> VP8StatusCode;
    fn WebPInitDecoderConfigInternal(_: *mut WebPDecoderConfig, _: c_int) -> c_int;
    pub fn WebPIDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> *mut WebPIDecoder;
    pub fn WebPDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPInitDecBuffer(buffer: *mut WebPDecBuffer) -> c_int {
    WebPInitDecBufferInternal(buffer, WEBP_DECODER_ABI_VERSION)
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPGetFeatures(
    data: *const u8,
    data_size: usize,
    features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    WebPGetFeaturesInternal(data, data_size, features, WEBP_DECODER_ABI_VERSION)
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPInitDecoderConfig(config: *mut WebPDecoderConfig) -> c_int {
    WebPInitDecoderConfigInternal(config, WEBP_DECODER_ABI_VERSION)
}
