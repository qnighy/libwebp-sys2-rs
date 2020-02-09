use std::os::raw::*;

pub use self::WebPEncCSP::*;
pub use self::WebPEncodingError::*;
pub use self::WebPImageHint::*;
pub use self::WebPPreset::*;

cfg_if! {
    if #[cfg(feature = "0.6")] {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x020E;
    } else if #[cfg(feature = "0.5")] {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x0209;
    } else {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x0202;
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPImageHint {
    WEBP_HINT_DEFAULT = 0,
    WEBP_HINT_PICTURE = 1,
    WEBP_HINT_PHOTO = 2,
    WEBP_HINT_GRAPH = 3,
}

impl WebPImageHint {
    pub const WEBP_HINT_LAST: c_int = 4;
}

pub const WEBP_HINT_LAST: c_int = WebPImageHint::WEBP_HINT_LAST;

#[allow(non_snake_case)]
#[repr(C)]
pub struct WebPConfig {
    pub lossless: c_int,
    pub quality: c_float,
    pub method: c_int,
    pub image_hint: WebPImageHint,
    pub target_size: c_int,
    pub target_PSNR: c_float,
    pub segments: c_int,
    pub sns_strength: c_int,
    pub filter_strength: c_int,
    pub filter_sharpness: c_int,
    pub filter_type: c_int,
    pub autofilter: c_int,
    pub alpha_compression: c_int,
    pub alpha_filtering: c_int,
    pub alpha_quality: c_int,
    pub pass: c_int,
    pub show_compressed: c_int,
    pub preprocessing: c_int,
    pub partitions: c_int,
    pub partition_limit: c_int,
    pub emulate_jpeg_size: c_int,
    pub thread_level: c_int,
    pub low_memory: c_int,
    #[cfg(feature = "0.5")]
    pub near_lossless: c_int,
    #[cfg(feature = "0.5")]
    pub exact: c_int,
    #[cfg(feature = "0.6")]
    pub use_delta_palette: c_int,
    #[cfg(feature = "0.6")]
    pub use_sharp_yuv: c_int,
    #[cfg(not(feature = "0.5"))]
    pub pad: [u32; 5],
    #[cfg(all(feature = "0.5", not(feature = "0.6")))]
    pub pad: [u32; 3],
    #[cfg(feature = "0.6")]
    pub pad: [u32; 2],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPPreset {
    WEBP_PRESET_DEFAULT = 0,
    WEBP_PRESET_PICTURE = 1,
    WEBP_PRESET_PHOTO = 2,
    WEBP_PRESET_DRAWING = 3,
    WEBP_PRESET_ICON = 4,
    WEBP_PRESET_TEXT = 5,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct WebPAuxStats {
    pub coded_size: c_int,
    pub PSNR: [c_float; 5],
    pub block_count: [c_int; 3],
    pub header_bytes: [c_int; 2],
    pub residual_bytes: [[c_int; 4]; 3],
    pub segment_size: [c_int; 4],
    pub segment_quant: [c_int; 4],
    pub segment_level: [c_int; 4],
    pub alpha_data_size: c_int,
    pub layer_data_size: c_int,
    pub lossless_features: u32,
    pub histogram_bits: c_int,
    pub transform_bits: c_int,
    pub cache_bits: c_int,
    pub palette_size: c_int,
    pub lossless_size: c_int,
    #[cfg(feature = "0.5")]
    pub lossless_hdr_size: c_int,
    #[cfg(feature = "0.5")]
    pub lossless_data_size: c_int,
    #[cfg(not(feature = "0.5"))]
    pub pad: [u32; 4],
    #[cfg(feature = "0.5")]
    pub pad: [u32; 2],
}

pub type WebPWriterFunction = Option<extern "C" fn(*const u8, usize, *const WebPPicture) -> c_int>;

pub type WebPProgressHook = Option<extern "C" fn(c_int, *const WebPPicture) -> c_int>;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPEncCSP {
    WEBP_YUV420 = 0,
    WEBP_YUV420A = 4,
}

pub const WEBP_CSP_UV_MASK: c_int = 3;
pub const WEBP_CSP_ALPHA_BIT: c_int = 4;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum WebPEncodingError {
    VP8_ENC_OK = 0,
    VP8_ENC_ERROR_OUT_OF_MEMORY = 1,
    VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY = 2,
    VP8_ENC_ERROR_NULL_PARAMETER = 3,
    VP8_ENC_ERROR_INVALID_CONFIGURATION = 4,
    VP8_ENC_ERROR_BAD_DIMENSION = 5,
    VP8_ENC_ERROR_PARTITION0_OVERFLOW = 6,
    VP8_ENC_ERROR_PARTITION_OVERFLOW = 7,
    VP8_ENC_ERROR_BAD_WRITE = 8,
    VP8_ENC_ERROR_FILE_TOO_BIG = 9,
    VP8_ENC_ERROR_USER_ABORT = 10,
    VP8_ENC_ERROR_LAST = 11,
}

#[repr(C)]
pub struct WebPMemoryWriter {
    pub mem: *mut u8,
    pub size: usize,
    pub max_size: usize,
    pub pad: [u32; 1],
}

pub const WEBP_MAX_DIMENSION: c_int = 16383;

#[repr(C)]
pub struct WebPPicture {
    pub use_argb: c_int,
    pub colorspace: WebPEncCSP,
    pub width: c_int,
    pub height: c_int,
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub y_stride: c_int,
    pub uv_stride: c_int,
    pub a: *mut u8,
    pub a_stride: c_int,
    pub pad1: [u32; 2],
    pub argb: *mut u32,
    pub argb_stride: c_int,
    pub pad2: [u32; 3],
    pub writer: WebPWriterFunction,
    pub custom_ptr: *mut c_void,
    pub extra_info_type: c_int,
    pub extra_info: *mut u8,
    pub stats: *mut WebPAuxStats,
    pub error_code: WebPEncodingError,
    pub progress_hook: WebPProgressHook,
    pub user_data: *mut c_void,
    pub pad3: [u32; 3],
    pub pad4: *mut u8,
    pub pad5: *mut u8,
    pub pad6: [u32; 8],
    memory_: *mut c_void,
    memory_argb_: *mut c_void,
    pad7: [*mut c_void; 2],
}

#[link(name = "webp")]
extern "C" {
    pub fn WebPGetEncoderVersion() -> c_int;
    pub fn WebPEncodeRGB(
        rgb: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeBGR(
        bgr: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeRGBA(
        rgba: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeBGRA(
        bgra: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeLosslessRGB(
        rgb: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeLosslessBGR(
        bgr: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeLosslessRGBA(
        rgba: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    pub fn WebPEncodeLosslessBGRA(
        bgra: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    // #[cfg(feature = "0.5")]
    // pub fn WebPFree(ptr: *mut c_void);  // see decode.rs
    fn WebPConfigInitInternal(_: *mut WebPConfig, _: WebPPreset, _: c_float, _: c_int) -> c_int;
    #[cfg(feature = "0.5")]
    pub fn WebPConfigLosslessPreset(config: *mut WebPConfig, level: c_int) -> c_int;
    pub fn WebPValidateConfig(config: *const WebPConfig) -> c_int;
    pub fn WebPMemoryWriterInit(writer: *mut WebPMemoryWriter);
    #[cfg(feature = "0.5")]
    pub fn WebPMemoryWriterClear(writer: *mut WebPMemoryWriter);
    pub fn WebPMemoryWrite(data: *const u8, data_size: usize, picture: *const WebPPicture)
        -> c_int;
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: c_int) -> c_int;
    pub fn WebPPictureAlloc(picture: *mut WebPPicture) -> c_int;
    pub fn WebPPictureFree(picture: *mut WebPPicture);
    pub fn WebPPictureCopy(src: *const WebPPicture, dst: *mut WebPPicture) -> c_int;
    #[cfg(feature = "0.6")]
    pub fn WebPPlaneDistortion(
        src: *const u8,
        src_stride: usize,
        ref_: *const u8,
        ref_stride: usize,
        width: c_int,
        height: c_int,
        x_step: usize,
        type_: c_int,
        distortion: *mut c_float,
        result: *mut c_float,
    ) -> c_int;
    pub fn WebPPictureDistortion(
        src: *const WebPPicture,
        ref_: *const WebPPicture,
        metric_type: c_int,
        result: *mut c_float,
    ) -> c_int;
    pub fn WebPPictureCrop(
        picture: *mut WebPPicture,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;
    pub fn WebPPictureView(
        src: *const WebPPicture,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        dst: *mut WebPPicture,
    ) -> c_int;
    pub fn WebPPictureIsView(picture: *const WebPPicture) -> c_int;
    pub fn WebPPictureRescale(pic: *mut WebPPicture, width: c_int, height: c_int) -> c_int;
    pub fn WebPPictureImportRGB(
        picture: *mut WebPPicture,
        rgb: *const u8,
        rgb_stride: c_int,
    ) -> c_int;
    pub fn WebPPictureImportRGBA(
        picture: *mut WebPPicture,
        rgba: *const u8,
        rgba_stride: c_int,
    ) -> c_int;
    pub fn WebPPictureImportRGBX(
        picture: *mut WebPPicture,
        rgbx: *const u8,
        rgbx_stride: c_int,
    ) -> c_int;
    pub fn WebPPictureImportBGR(
        picture: *mut WebPPicture,
        bgr: *const u8,
        bgr_stride: c_int,
    ) -> c_int;
    pub fn WebPPictureImportBGRA(
        picture: *mut WebPPicture,
        bgra: *const u8,
        bgra_stride: c_int,
    ) -> c_int;
    pub fn WebPPictureImportBGRX(
        picture: *mut WebPPicture,
        bgrx: *const u8,
        bgrx_stride: c_int,
    ) -> c_int;
    pub fn WebPPictureARGBToYUVA(picture: *mut WebPPicture, colorspace: WebPEncCSP) -> c_int;
    pub fn WebPPictureARGBToYUVADithered(
        picture: *mut WebPPicture,
        colorspace: WebPEncCSP,
        dithering: c_float,
    ) -> c_int;
    #[cfg(feature = "0.5")]
    pub fn WebPPictureSmartARGBToYUVA(picture: *mut WebPPicture) -> c_int;
    pub fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> c_int;
    pub fn WebPCleanupTransparentArea(picture: *mut WebPPicture);
    pub fn WebPPictureHasTransparency(picture: *const WebPPicture) -> c_int;
    pub fn WebPBlendAlpha(pic: *mut WebPPicture, background_rgb: u32);
    pub fn WebPEncode(config: *const WebPConfig, picture: *mut WebPPicture) -> c_int;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPConfigInit(config: *mut WebPConfig) -> c_int {
    WebPConfigInitInternal(
        config,
        WEBP_PRESET_DEFAULT,
        75_f32 as c_float,
        WEBP_ENCODER_ABI_VERSION,
    )
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPConfigPreset(
    config: *mut WebPConfig,
    preset: WebPPreset,
    quality: c_float,
) -> c_int {
    WebPConfigInitInternal(config, preset, quality, WEBP_ENCODER_ABI_VERSION)
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn WebPPictureInit(picture: *mut WebPPicture) -> c_int {
    WebPPictureInitInternal(picture, WEBP_ENCODER_ABI_VERSION)
}
