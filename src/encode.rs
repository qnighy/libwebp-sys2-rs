use std::os::raw::*;

// MAJOR(8b) + MINOR(8b)
cfg_if! {
    if #[cfg(feature = "1_5")] {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x0210;
    } else if #[cfg(feature = "1_1")] {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x020F;
    } else if #[cfg(feature = "0_6")] {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x020E;
    } else if #[cfg(feature = "0_5")] {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x0209;
    } else {
        pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x0202;
    }
}

/// Image characteristics hint for the underlying encoder.
#[allow(non_camel_case_types)]
pub type WebPImageHint = u32;

/// default preset.
pub const WEBP_HINT_DEFAULT: WebPImageHint = 0;
/// digital picture, like portrait, inner shot
pub const WEBP_HINT_PICTURE: WebPImageHint = 1;
/// outdoor photograph, with natural lighting
pub const WEBP_HINT_PHOTO: WebPImageHint = 2;
/// Discrete tone image (graph, map-tile etc).
pub const WEBP_HINT_GRAPH: WebPImageHint = 3;
pub const WEBP_HINT_LAST: WebPImageHint = 4;

/// Compression parameters.
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPConfig {
    /// Lossless encoding (0=lossy(default), 1=lossless).
    pub lossless: c_int,
    /// between 0 and 100. For lossy, 0 gives the smallest
    /// size and 100 the largest. For lossless, this
    /// parameter is the amount of effort put into the
    /// compression: 0 is the fastest but gives larger
    /// files compared to the slowest, but best, 100.
    pub quality: c_float,
    /// quality/speed trade-off (0=fast, 6=slower-better)
    pub method: c_int,
    /// Hint for image type (lossless only for now).
    pub image_hint: WebPImageHint,
    /// if non-zero, set the desired target size in bytes.
    /// Takes precedence over the `compression` parameter.
    pub target_size: c_int,
    /// if non-zero, specifies the minimal distortion to
    /// try to achieve. Takes precedence over target_size.
    pub target_PSNR: c_float,
    /// maximum number of segments to use, in \[1..4\]
    pub segments: c_int,
    /// Spatial Noise Shaping. 0=off, 100=maximum.
    pub sns_strength: c_int,
    /// range: \[0 = off .. 100 = strongest\]
    pub filter_strength: c_int,
    /// range: \[0 = off .. 7 = least sharp\]
    pub filter_sharpness: c_int,
    /// filtering type: 0 = simple, 1 = strong (only used
    /// if filter_strength > 0 or autofilter > 0)
    pub filter_type: c_int,
    /// Auto adjust filter's strength \[0 = off, 1 = on\]
    pub autofilter: c_int,
    /// Algorithm for encoding the alpha plane (0 = none,
    /// 1 = compressed with WebP lossless). Default is 1.
    pub alpha_compression: c_int,
    /// Predictive filtering method for alpha plane.
    ///  0: none, 1: fast, 2: best. Default if 1.
    pub alpha_filtering: c_int,
    /// Between 0 (smallest size) and 100 (lossless).
    /// Default is 100.
    pub alpha_quality: c_int,
    /// number of entropy-analysis passes (in \[1..10\]).
    pub pass: c_int,
    /// if true, export the compressed picture back.
    /// In-loop filtering is not applied.
    pub show_compressed: c_int,
    /// preprocessing filter:
    /// 0=none, 1=segment-smooth, 2=pseudo-random dithering
    pub preprocessing: c_int,
    /// log2(number of token partitions) in \[0..3\]. Default
    /// is set to 0 for easier progressive decoding.
    pub partitions: c_int,
    /// quality degradation allowed to fit the 512k limit
    /// on prediction modes coding (0: no degradation,
    /// 100: maximum possible degradation).
    pub partition_limit: c_int,
    /// If true, compression parameters will be remapped
    /// to better match the expected output size from
    /// JPEG compression. Generally, the output size will
    /// be similar but the degradation will be lower.
    pub emulate_jpeg_size: c_int,
    /// If non-zero, try and use multi-threaded encoding.
    pub thread_level: c_int,
    /// If set, reduce memory usage (but increase CPU use).
    pub low_memory: c_int,
    /// Near lossless encoding \[0 = max loss .. 100 = off
    /// (default)\].
    #[cfg(feature = "0_5")]
    pub near_lossless: c_int,
    /// if non-zero, preserve the exact RGB values under
    /// transparent area. Otherwise, discard this invisible
    /// RGB information for better compression. The default
    /// value is 0.
    #[cfg(feature = "0_5")]
    pub exact: c_int,
    /// reserved
    #[cfg(feature = "0_6")]
    pub use_delta_palette: c_int,
    /// if needed, use sharp (and slow) RGB->YUV conversion
    #[cfg(feature = "0_6")]
    pub use_sharp_yuv: c_int,
    /// minimum permissible quality factor
    #[cfg(feature = "1_2")]
    pub qmin: c_int,
    /// maximum permissible quality factor
    #[cfg(feature = "1_2")]
    pub qmax: c_int,
    #[cfg(not(feature = "0_5"))]
    #[doc(hidden)]
    pub pad: [u32; 5],
    #[cfg(all(feature = "0_5", not(feature = "0_6")))]
    #[doc(hidden)]
    pub pad: [u32; 3],
    #[cfg(all(feature = "0_6", not(feature = "1_2")))]
    #[doc(hidden)]
    pub pad: [u32; 2],
}

/// Enumerate some predefined settings for WebPConfig, depending on the type
/// of source picture. These presets are used when calling WebPConfigPreset().
#[allow(non_camel_case_types)]
pub type WebPPreset = u32;

/// default preset.
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
/// digital picture, like portrait, inner shot
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
/// outdoor photograph, with natural lighting
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
/// hand or line drawing, with high-contrast details
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
/// small-sized colorful images
pub const WEBP_PRESET_ICON: WebPPreset = 4;
/// text-like
pub const WEBP_PRESET_TEXT: WebPPreset = 5;

/// Structure for storing auxiliary statistics.
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPAuxStats {
    /// final size
    pub coded_size: c_int,
    /// peak-signal-to-noise ratio for Y/U/V/All/Alpha
    pub PSNR: [c_float; 5],
    /// number of intra4/intra16/skipped macroblocks
    pub block_count: [c_int; 3],
    /// approximate number of bytes spent for header
    /// and mode-partition #0
    pub header_bytes: [c_int; 2],
    /// approximate number of bytes spent for
    /// DC/AC/uv coefficients for each (0..3) segments.
    pub residual_bytes: [[c_int; 4]; 3],
    /// number of macroblocks in each segments
    pub segment_size: [c_int; 4],
    /// quantizer values for each segments
    pub segment_quant: [c_int; 4],
    /// filtering strength for each segments \[0..63\]
    pub segment_level: [c_int; 4],
    /// size of the transparency data
    pub alpha_data_size: c_int,
    /// size of the enhancement layer data
    pub layer_data_size: c_int,
    // lossless encoder statistics
    /// bit0:predictor bit1:cross-color transform
    /// bit2:subtract-green bit3:color indexing
    pub lossless_features: u32,
    /// number of precision bits of histogram
    pub histogram_bits: c_int,
    /// precision bits for predictor transform
    pub transform_bits: c_int,
    /// number of bits for color cache lookup
    pub cache_bits: c_int,
    /// number of color in palette, if used
    pub palette_size: c_int,
    /// final lossless size
    pub lossless_size: c_int,
    /// lossless header (transform, huffman etc) size
    #[cfg(feature = "0_5")]
    pub lossless_hdr_size: c_int,
    /// lossless image data size
    #[cfg(feature = "0_5")]
    pub lossless_data_size: c_int,
    /// precision bits for cross-color transform
    #[cfg(feature = "1_5")]
    pub cross_color_transform_bits: c_int,
    /// padding for later use
    #[cfg(not(feature = "0_5"))]
    #[doc(hidden)]
    pub pad: [u32; 4],
    /// padding for later use
    #[cfg(all(feature = "0_5", not(feature = "1_5")))]
    #[doc(hidden)]
    pub pad: [u32; 2],
    /// padding for later use
    #[cfg(feature = "1_5")]
    #[doc(hidden)]
    pub pad: [u32; 1],
}

/// Signature for output function. Should return true if writing was successful.
/// data/data_size is the segment of data to write, and `picture` is for
/// reference (and so one can make use of picture->custom_ptr).
pub type WebPWriterFunction = Option<extern "C" fn(*const u8, usize, *const WebPPicture) -> c_int>;

/// Progress hook, called from time to time to report progress. It can return
/// false to request an abort of the encoding process, or true otherwise if
/// everything is OK.
pub type WebPProgressHook = Option<extern "C" fn(c_int, *const WebPPicture) -> c_int>;

/// Color spaces.
#[allow(non_camel_case_types)]
pub type WebPEncCSP = u32;

// chroma sampling
/// 4:2:0
pub const WEBP_YUV420: WebPEncCSP = 0;
/// alpha channel variant
pub const WEBP_YUV420A: WebPEncCSP = 4;

/// bit-mask to get the UV sampling factors
pub const WEBP_CSP_UV_MASK: WebPEncCSP = 3;
/// bit that is set if alpha is present
pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = 4;

/// Encoding error conditions.
#[allow(non_camel_case_types)]
pub type WebPEncodingError = u32;

pub const VP8_ENC_OK: WebPEncodingError = 0;
/// memory error allocating objects
pub const VP8_ENC_ERROR_OUT_OF_MEMORY: WebPEncodingError = 1;
/// memory error while flushing bits
pub const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: WebPEncodingError = 2;
/// a pointer parameter is NULL
pub const VP8_ENC_ERROR_NULL_PARAMETER: WebPEncodingError = 3;
/// configuration is invalid
pub const VP8_ENC_ERROR_INVALID_CONFIGURATION: WebPEncodingError = 4;
/// picture has invalid width/height
pub const VP8_ENC_ERROR_BAD_DIMENSION: WebPEncodingError = 5;
/// partition is bigger than 512k
pub const VP8_ENC_ERROR_PARTITION0_OVERFLOW: WebPEncodingError = 6;
/// partition is bigger than 16M
pub const VP8_ENC_ERROR_PARTITION_OVERFLOW: WebPEncodingError = 7;
/// error while flushing bytes
pub const VP8_ENC_ERROR_BAD_WRITE: WebPEncodingError = 8;
/// file is bigger than 4G
pub const VP8_ENC_ERROR_FILE_TOO_BIG: WebPEncodingError = 9;
/// abort request by user
pub const VP8_ENC_ERROR_USER_ABORT: WebPEncodingError = 10;
/// list terminator. always last.
pub const VP8_ENC_ERROR_LAST: WebPEncodingError = 11;

/// WebPMemoryWrite: a special WebPWriterFunction that writes to memory using
/// the following WebPMemoryWriter object (to be set as a custom_ptr).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPMemoryWriter {
    pub mem: *mut u8,
    pub size: usize,
    pub max_size: usize,
    #[doc(hidden)]
    pub pad: [u32; 1],
}

// maximum width/height allowed (inclusive), in pixels
pub const WEBP_MAX_DIMENSION: c_int = 16383;

/// Main exchange structure (input samples, output bytes, statistics)
///
/// Once WebPPictureInit() has been called, it's ok to make all the INPUT fields
/// (use_argb, y/u/v, argb, ...) point to user-owned data, even if
/// WebPPictureAlloc() has been called. Depending on the value use_argb,
/// it's guaranteed that either \*argb or \*y/\*u/\*v content will be kept untouched.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPPicture {
    //   INPUT
    //////////////
    /// Main flag for encoder selecting between ARGB or YUV input.
    /// It is recommended to use ARGB input (\*argb, argb_stride) for lossless
    /// compression, and YUV input (\*y, \*u, \*v, etc.) for lossy compression
    /// since these are the respective native colorspace for these formats.
    pub use_argb: c_int,

    // YUV input (mostly used for input to lossy compression)
    /// colorspace: should be YUV420 for now (=Y'CbCr).
    pub colorspace: WebPEncCSP,
    /// dimensions (less or equal to WEBP_MAX_DIMENSION)
    pub width: c_int,
    /// dimensions (less or equal to WEBP_MAX_DIMENSION)
    pub height: c_int,
    /// pointers to luma/chroma planes.
    pub y: *mut u8,
    /// pointers to luma/chroma planes.
    pub u: *mut u8,
    /// pointers to luma/chroma planes.
    pub v: *mut u8,
    /// luma/chroma strides.
    pub y_stride: c_int,
    /// luma/chroma strides.
    pub uv_stride: c_int,
    /// pointer to the alpha plane
    pub a: *mut u8,
    /// stride of the alpha plane
    pub a_stride: c_int,
    /// padding for later use
    #[doc(hidden)]
    pub pad1: [u32; 2],

    // ARGB input (mostly used for input to lossless compression)
    /// Pointer to argb (32 bit) plane.
    pub argb: *mut u32,
    /// This is stride in pixels units, not bytes.
    pub argb_stride: c_int,
    /// padding for later use
    #[doc(hidden)]
    pub pad2: [u32; 3],

    //   OUTPUT
    ///////////////
    // Byte-emission hook, to store compressed bytes as they are ready.
    /// can be NULL
    pub writer: WebPWriterFunction,
    /// can be used by the writer.
    pub custom_ptr: *mut c_void,

    // map for extra information (only for lossy compression mode)
    /// 1: intra type, 2: segment, 3: quant
    /// 4: intra-16 prediction mode,
    /// 5: chroma prediction mode,
    /// 6: bit cost, 7: distortion
    pub extra_info_type: c_int,
    /// if not NULL, points to an array of size
    /// ((width + 15) / 16) \* ((height + 15) / 16) that
    /// will be filled with a macroblock map, depending
    /// on extra_info_type.                                
    pub extra_info: *mut u8,

    //   STATS AND REPORTS
    ///////////////////////////
    /// Pointer to side statistics (updated only if not NULL)
    pub stats: *mut WebPAuxStats,
    /// Error code for the latest error encountered during encoding
    pub error_code: WebPEncodingError,
    /// If not NULL, report progress during encoding.
    pub progress_hook: WebPProgressHook,
    /// this field is free to be set to any value and
    /// used during callbacks (like progress-report e.g.).
    pub user_data: *mut c_void,
    /// padding for later use
    #[doc(hidden)]
    pub pad3: [u32; 3],
    /// Unused for now
    #[doc(hidden)]
    pub pad4: *mut u8,
    /// Unused for now
    #[doc(hidden)]
    pub pad5: *mut u8,
    /// padding for later use
    #[doc(hidden)]
    pub pad6: [u32; 8],

    // PRIVATE FIELDS
    ////////////////////
    /// row chunk of memory for yuva planes
    #[doc(hidden)]
    pub memory_: *mut c_void,
    /// and for argb too.
    #[doc(hidden)]
    pub memory_argb_: *mut c_void,
    /// padding for later use
    #[doc(hidden)]
    pub pad7: [*mut c_void; 2],
}

unsafe extern "C" {
    /// Return the encoder's version number, packed in hexadecimal using 8bits for
    /// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
    pub fn WebPGetEncoderVersion() -> c_int;
    /// Returns the size of the compressed data (pointed to by `*output``), or 0 if
    /// an error occurred. The compressed data must be released by the caller
    /// using the call `WebPFree(*output)`.
    /// These functions compress using the lossy format, and the quality_factor
    /// can go from 0 (smaller output, lower quality) to 100 (best quality,
    /// larger output).
    pub fn WebPEncodeRGB(
        rgb: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    /// Returns the size of the compressed data (pointed to by `*output``), or 0 if
    /// an error occurred. The compressed data must be released by the caller
    /// using the call `WebPFree(*output)`.
    /// These functions compress using the lossy format, and the quality_factor
    /// can go from 0 (smaller output, lower quality) to 100 (best quality,
    /// larger output).
    pub fn WebPEncodeBGR(
        bgr: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    /// Returns the size of the compressed data (pointed to by `*output``), or 0 if
    /// an error occurred. The compressed data must be released by the caller
    /// using the call `WebPFree(*output)`.
    /// These functions compress using the lossy format, and the quality_factor
    /// can go from 0 (smaller output, lower quality) to 100 (best quality,
    /// larger output).
    pub fn WebPEncodeRGBA(
        rgba: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    /// Returns the size of the compressed data (pointed to by `*output``), or 0 if
    /// an error occurred. The compressed data must be released by the caller
    /// using the call `WebPFree(*output)`.
    /// These functions compress using the lossy format, and the quality_factor
    /// can go from 0 (smaller output, lower quality) to 100 (best quality,
    /// larger output).
    pub fn WebPEncodeBGRA(
        bgra: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        quality_factor: c_float,
        output: *mut *mut u8,
    ) -> usize;
    /// Equivalent to [WebPEncodeRGB], but compressing in a
    /// lossless manner. Files are usually larger than lossy format, but will
    /// not suffer any compression loss.
    /// Note these functions, like the lossy versions, use the library's default
    /// settings. For lossless this means `exact` is disabled. RGB values in
    /// transparent areas will be modified to improve compression. To avoid this,
    /// use WebPEncode() and set WebPConfig::exact to 1.
    pub fn WebPEncodeLosslessRGB(
        rgb: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    /// Equivalent to [WebPEncodeBGR], but compressing in a
    /// lossless manner. Files are usually larger than lossy format, but will
    /// not suffer any compression loss.
    /// Note these functions, like the lossy versions, use the library's default
    /// settings. For lossless this means `exact` is disabled. RGB values in
    /// transparent areas will be modified to improve compression. To avoid this,
    /// use WebPEncode() and set WebPConfig::exact to 1.
    pub fn WebPEncodeLosslessBGR(
        bgr: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    /// Equivalent to [WebPEncodeRGBA], but compressing in a
    /// lossless manner. Files are usually larger than lossy format, but will
    /// not suffer any compression loss.
    /// Note these functions, like the lossy versions, use the library's default
    /// settings. For lossless this means `exact` is disabled. RGB values in
    /// transparent areas will be modified to improve compression. To avoid this,
    /// use WebPEncode() and set WebPConfig::exact to 1.
    pub fn WebPEncodeLosslessRGBA(
        rgba: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    /// Equivalent to [WebPEncodeBGRA], but compressing in a
    /// lossless manner. Files are usually larger than lossy format, but will
    /// not suffer any compression loss.
    /// Note these functions, like the lossy versions, use the library's default
    /// settings. For lossless this means `exact` is disabled. RGB values in
    /// transparent areas will be modified to improve compression. To avoid this,
    /// use WebPEncode() and set WebPConfig::exact to 1.
    pub fn WebPEncodeLosslessBGRA(
        bgra: *const u8,
        width: c_int,
        height: c_int,
        stride: c_int,
        output: *mut *mut u8,
    ) -> usize;
    /// Internal, version-checked, entry point
    #[doc(hidden)]
    #[must_use]
    pub fn WebPConfigInitInternal(_: *mut WebPConfig, _: WebPPreset, _: c_float, _: c_int)
    -> c_int;
    /// Activate the lossless compression mode with the desired efficiency level
    /// between 0 (fastest, lowest compression) and 9 (slower, best compression).
    /// A good default level is `6`, providing a fair tradeoff between compression
    /// speed and final compressed size.
    /// This function will overwrite several fields from config: `method`, `quality`
    /// and `lossless`. Returns false in case of parameter error.
    #[cfg(feature = "0_5")]
    #[must_use]
    pub fn WebPConfigLosslessPreset(config: *mut WebPConfig, level: c_int) -> c_int;
    /// Returns true if `config` is non-NULL and all configuration parameters are
    /// within their valid ranges.
    #[must_use]
    pub fn WebPValidateConfig(config: *const WebPConfig) -> c_int;
    /// The following must be called first before any use.
    pub fn WebPMemoryWriterInit(writer: *mut WebPMemoryWriter);
    /// The following must be called to deallocate writer->mem memory. The `writer`
    /// object itself is not deallocated.
    #[cfg(feature = "0_5")]
    pub fn WebPMemoryWriterClear(writer: *mut WebPMemoryWriter);
    /// The custom writer to be used with WebPMemoryWriter as custom_ptr. Upon
    /// completion, writer.mem and writer.size will hold the coded data.
    /// writer.mem must be freed by calling WebPMemoryWriterClear.
    #[must_use]
    pub fn WebPMemoryWrite(data: *const u8, data_size: usize, picture: *const WebPPicture)
    -> c_int;
    /// Internal, version-checked, entry point
    #[doc(hidden)]
    #[must_use]
    pub fn WebPPictureInitInternal(_: *mut WebPPicture, _: c_int) -> c_int;
    /// Convenience allocation / deallocation based on picture->width/height:
    /// Allocate y/u/v buffers as per colorspace/width/height specification.
    /// Note! This function will free the previous buffer if needed.
    /// Returns false in case of memory error.
    #[must_use]
    pub fn WebPPictureAlloc(picture: *mut WebPPicture) -> c_int;
    /// Release the memory allocated by WebPPictureAlloc() or WebPPictureImport\*().
    /// Note that this function does _not_ free the memory used by the `picture`
    /// object itself.
    /// Besides memory (which is reclaimed) all other fields of `picture` are
    /// preserved.
    pub fn WebPPictureFree(picture: *mut WebPPicture);
    /// Copy the pixels of \*src into \*dst, using WebPPictureAlloc. Upon return, \*dst
    /// will fully own the copied pixels (this is not a view). The `dst` picture need
    /// not be initialized as its content is overwritten.
    /// Returns false in case of memory allocation error.
    #[must_use]
    pub fn WebPPictureCopy(src: *const WebPPicture, dst: *mut WebPPicture) -> c_int;
    /// Compute the single distortion for packed planes of samples.
    /// `src` will be compared to `ref`, and the raw distortion stored into
    /// `*distortion`. The refined metric (log(MSE), log(1 - ssim),...) will be
    /// stored in `*result`.
    /// `x_step` is the horizontal stride (in bytes) between samples.
    /// `src/ref_stride` is the byte distance between rows.
    /// Returns false in case of error (bad parameter, memory allocation error, ...).
    #[cfg(feature = "0_6")]
    #[must_use]
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
    /// Compute PSNR, SSIM or LSIM distortion metric between two pictures. Results
    /// are in dB, stored in result\[\] in the B/G/R/A/All order. The distortion is
    /// always performed using ARGB samples. Hence if the input is YUV(A), the
    /// picture will be internally converted to ARGB (just for the measurement).
    /// Warning: this function is rather CPU-intensive.
    #[must_use]
    pub fn WebPPictureDistortion(
        src: *const WebPPicture,
        ref_: *const WebPPicture,
        metric_type: c_int,
        result: *mut c_float,
    ) -> c_int;
    /// self-crops a picture to the rectangle defined by top/left/width/height.
    /// Returns false in case of memory allocation error, or if the rectangle is
    /// outside of the source picture.
    /// The rectangle for the view is defined by the top-left corner pixel
    /// coordinates (left, top) as well as its width and height. This rectangle
    /// must be fully be comprised inside the `src` source picture. If the source
    /// picture uses the YUV420 colorspace, the top and left coordinates will be
    /// snapped to even values.
    #[must_use]
    pub fn WebPPictureCrop(
        picture: *mut WebPPicture,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;
    /// Extracts a view from `src` picture into `dst`. The rectangle for the view
    /// is defined by the top-left corner pixel coordinates (left, top) as well
    /// as its width and height. This rectangle must be fully be comprised inside
    /// the `src` source picture. If the source picture uses the YUV420 colorspace,
    /// the top and left coordinates will be snapped to even values.
    /// Picture `src` must out-live `dst` picture. Self-extraction of view is allowed
    /// (`src` equal to `dst`) as a mean of fast-cropping (but note that doing so,
    /// the original dimension will be lost). Picture `dst` need not be initialized
    /// with WebPPictureInit() if it is different from `src`, since its content will
    /// be overwritten.
    /// Returns false in case of invalid parameters.
    #[must_use]
    pub fn WebPPictureView(
        src: *const WebPPicture,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        dst: *mut WebPPicture,
    ) -> c_int;
    /// Returns true if the `picture` is actually a view and therefore does
    /// not own the memory for pixels.
    pub fn WebPPictureIsView(picture: *const WebPPicture) -> c_int;
    /// Rescale a picture to new dimension width x height.
    /// If either `width` or `height` (but not both) is 0 the corresponding
    /// dimension will be calculated preserving the aspect ratio.
    /// No gamma correction is applied.
    /// Returns false in case of error (invalid parameter or insufficient memory).
    #[must_use]
    pub fn WebPPictureRescale(picture: *mut WebPPicture, width: c_int, height: c_int) -> c_int;
    /// Colorspace conversion function to import RGB samples.
    /// Previous buffer will be free'd, if any.
    /// \*rgb buffer should have a size of at least height \* rgb_stride.
    /// Returns false in case of memory error.
    #[must_use]
    pub fn WebPPictureImportRGB(
        picture: *mut WebPPicture,
        rgb: *const u8,
        rgb_stride: c_int,
    ) -> c_int;
    /// Same as [WebPPictureImportRGB], but for RGBA buffer.
    #[must_use]
    pub fn WebPPictureImportRGBA(
        picture: *mut WebPPicture,
        rgba: *const u8,
        rgba_stride: c_int,
    ) -> c_int;
    /// Same as [WebPPictureImportRGB], but for RGBA buffer. Imports the RGB direct from the 32-bit format
    /// input buffer ignoring the alpha channel. Avoids needing to copy the data
    /// to a temporary 24-bit RGB buffer to import the RGB only.
    #[must_use]
    pub fn WebPPictureImportRGBX(
        picture: *mut WebPPicture,
        rgbx: *const u8,
        rgbx_stride: c_int,
    ) -> c_int;
    /// Variants of the above [WebPPictureImportRGB], but taking BGR(A|X) input.
    #[must_use]
    pub fn WebPPictureImportBGR(
        picture: *mut WebPPicture,
        bgr: *const u8,
        bgr_stride: c_int,
    ) -> c_int;
    /// Variants of the above [WebPPictureImportRGB], but taking BGR(A|X) input.
    #[must_use]
    pub fn WebPPictureImportBGRA(
        picture: *mut WebPPicture,
        bgra: *const u8,
        bgra_stride: c_int,
    ) -> c_int;
    /// Variants of the above [WebPPictureImportRGB], but taking BGR(A|X) input.
    #[must_use]
    pub fn WebPPictureImportBGRX(
        picture: *mut WebPPicture,
        bgrx: *const u8,
        bgrx_stride: c_int,
    ) -> c_int;
    /// Converts picture->argb data to the YUV420A format. The `colorspace`
    /// parameter is deprecated and should be equal to WEBP_YUV420.
    /// Upon return, picture->use_argb is set to false. The presence of real
    /// non-opaque transparent values is detected, and `colorspace` will be
    /// adjusted accordingly. Note that this method is lossy.
    /// Returns false in case of error.
    #[must_use]
    pub fn WebPPictureARGBToYUVA(picture: *mut WebPPicture, colorspace: WebPEncCSP) -> c_int;
    /// Same as WebPPictureARGBToYUVA(), but the conversion is done using
    /// pseudo-random dithering with a strength `dithering` between
    /// 0.0 (no dithering) and 1.0 (maximum dithering). This is useful
    /// for photographic picture.
    #[must_use]
    pub fn WebPPictureARGBToYUVADithered(
        picture: *mut WebPPicture,
        colorspace: WebPEncCSP,
        dithering: c_float,
    ) -> c_int;
    /// Performs `sharp` RGBA->YUVA420 downsampling and colorspace conversion
    /// Downsampling is handled with extra care in case of color clipping. This
    /// method is roughly 2x slower than WebPPictureARGBToYUVA() but produces better
    /// and sharper YUV representation.
    /// Returns false in case of error.
    #[cfg(feature = "0_6")]
    #[must_use]
    pub fn WebPPictureSharpARGBToYUVA(picture: *mut WebPPicture) -> c_int;
    /// kept for backward compatibility:
    #[cfg(feature = "0_5")]
    #[must_use]
    pub fn WebPPictureSmartARGBToYUVA(picture: *mut WebPPicture) -> c_int;
    /// Converts picture->yuv to picture->argb and sets picture->use_argb to true.
    /// The input format must be YUV_420 or YUV_420A. The conversion from YUV420 to
    /// ARGB incurs a small loss too.
    /// Note that the use of this colorspace is discouraged if one has access to the
    /// raw ARGB samples, since using YUV420 is comparatively lossy.
    /// Returns false in case of error.
    #[must_use]
    pub fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> c_int;
    /// Helper function: given a width x height plane of RGBA or YUV(A) samples
    /// clean-up or smoothen the YUV or RGB samples under fully transparent area,
    /// to help compressibility (no guarantee, though).
    pub fn WebPCleanupTransparentArea(picture: *mut WebPPicture);
    /// Scan the picture `picture` for the presence of non fully opaque alpha values.
    /// Returns true in such case. Otherwise returns false (indicating that the
    /// alpha plane can be ignored altogether e.g.).
    pub fn WebPPictureHasTransparency(picture: *const WebPPicture) -> c_int;
    /// Remove the transparency information (if present) by blending the color with
    /// the background color `background_rgb` (specified as 24bit RGB triplet).
    /// After this call, all alpha values are reset to 0xff.
    pub fn WebPBlendAlpha(picture: *mut WebPPicture, background_rgb: u32);
    /// Main encoding call, after config and picture have been initialized.
    /// `picture` must be less than 16384x16384 in dimension (cf WEBP_MAX_DIMENSION),
    /// and the `config` object must be a valid one.
    /// Returns false in case of error, true otherwise.
    /// In case of error, picture->error_code is updated accordingly.
    /// `picture` can hold the source samples in both YUV(A) or ARGB input, depending
    /// on the value of `picture->use_argb`. It is highly recommended to use
    /// the former for lossy encoding, and the latter for lossless encoding
    /// (when config.lossless is true). Automatic conversion from one format to
    /// another is provided but they both incur some loss.
    #[must_use]
    pub fn WebPEncode(config: *const WebPConfig, picture: *mut WebPPicture) -> c_int;
}

/// Should always be called, to initialize a fresh WebPConfig structure before
/// modification. Returns false in case of version mismatch. WebPConfigInit()
/// must have succeeded before using the `config` object.
/// Note that the default values are lossless=0 and quality=75.
#[allow(non_snake_case)]
#[must_use]
#[inline]
pub unsafe extern "C" fn WebPConfigInit(config: *mut WebPConfig) -> c_int {
    unsafe {
        WebPConfigInitInternal(
            config,
            WEBP_PRESET_DEFAULT,
            75_f32 as c_float,
            WEBP_ENCODER_ABI_VERSION,
        )
    }
}

/// This function will initialize the configuration according to a predefined
/// set of parameters (referred to by `preset`) and a given quality factor.
/// This function can be called as a replacement to WebPConfigInit(). Will
/// return false in case of error.
#[allow(non_snake_case)]
#[must_use]
#[inline]
pub unsafe extern "C" fn WebPConfigPreset(
    config: *mut WebPConfig,
    preset: WebPPreset,
    quality: c_float,
) -> c_int {
    unsafe { WebPConfigInitInternal(config, preset, quality, WEBP_ENCODER_ABI_VERSION) }
}

/// Should always be called, to initialize the structure. Returns false in case
/// of version mismatch. WebPPictureInit() must have succeeded before using the
/// `picture` object.
/// Note that, by default, use_argb is false and colorspace is WEBP_YUV420.
#[allow(non_snake_case)]
#[must_use]
#[inline]
pub unsafe extern "C" fn WebPPictureInit(picture: *mut WebPPicture) -> c_int {
    unsafe { WebPPictureInitInternal(picture, WEBP_ENCODER_ABI_VERSION) }
}
