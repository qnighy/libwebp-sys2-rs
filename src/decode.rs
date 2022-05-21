use std::os::raw::*;
use std::ptr;

// MAJOR(8b) + MINOR(8b)
cfg_if! {
    if #[cfg(feature = "1_1")] {
        pub const WEBP_DECODER_ABI_VERSION: c_int = 0x0209;
    } else if #[cfg(feature = "0_5")] {
        pub const WEBP_DECODER_ABI_VERSION: c_int = 0x0208;
    } else {
        pub const WEBP_DECODER_ABI_VERSION: c_int = 0x0203;
    }
}

#[cfg(feature = "extern-types")]
extern "C" {
    pub type WebPIDecoder;
}

#[cfg(not(feature = "extern-types"))]
#[repr(C)]
pub struct WebPIDecoder(c_void);

// Colorspaces
// Note: the naming describes the byte-ordering of packed samples in memory.
// For instance, MODE_BGRA relates to samples ordered as B,G,R,A,B,G,R,A,...
// Non-capital names (e.g.:MODE_Argb) relates to pre-multiplied RGB channels.
// RGBA-4444 and RGB-565 colorspaces are represented by following byte-order:
// RGBA-4444: [r3 r2 r1 r0 g3 g2 g1 g0], [b3 b2 b1 b0 a3 a2 a1 a0], ...
// RGB-565: [r4 r3 r2 r1 r0 g5 g4 g3], [g2 g1 g0 b4 b3 b2 b1 b0], ...
// In the case WEBP_SWAP_16BITS_CSP is defined, the bytes are swapped for
// these two modes:
// RGBA-4444: [b3 b2 b1 b0 a3 a2 a1 a0], [r3 r2 r1 r0 g3 g2 g1 g0], ...
// RGB-565: [g2 g1 g0 b4 b3 b2 b1 b0], [r4 r3 r2 r1 r0 g5 g4 g3], ...

#[allow(non_camel_case_types)]
pub type WEBP_CSP_MODE = u32;

pub const MODE_RGB: WEBP_CSP_MODE = 0;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
// RGB-premultiplied transparent modes (alpha value is preserved)
#[allow(non_upper_case_globals)]
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
#[allow(non_upper_case_globals)]
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
#[allow(non_upper_case_globals)]
pub const MODE_Argb: WEBP_CSP_MODE = 9;
#[allow(non_upper_case_globals)]
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
// YUV modes must come after RGB ones.
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_LAST: WEBP_CSP_MODE = 13;

// Some useful macros:

#[allow(non_snake_case)]
#[inline]
pub extern "C" fn WebPIsPremultipliedMode(mode: WEBP_CSP_MODE) -> c_int {
    (mode == MODE_rgbA || mode == MODE_bgrA || mode == MODE_Argb || mode == MODE_rgbA_4444) as c_int
}

#[allow(non_snake_case)]
#[inline]
pub extern "C" fn WebPIsAlphaMode(mode: WEBP_CSP_MODE) -> c_int {
    (mode == MODE_RGBA
        || mode == MODE_BGRA
        || mode == MODE_ARGB
        || mode == MODE_RGBA_4444
        || mode == MODE_YUVA
        || WebPIsPremultipliedMode(mode) != 0) as c_int
}

#[allow(non_snake_case)]
#[inline]
pub extern "C" fn WebPIsRGBMode(mode: WEBP_CSP_MODE) -> c_int {
    (mode < MODE_YUV) as c_int
}

//------------------------------------------------------------------------------
// WebPDecBuffer: Generic structure for describing the output sample buffer.

/// view as RGBA
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPRGBABuffer {
    /// pointer to RGBA samples
    pub rgba: *mut u8,
    /// stride in bytes from one scanline to the next.
    pub stride: c_int,
    /// total size of the *rgba buffer.
    pub size: usize,
}

/// view as YUVA
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPYUVABuffer {
    /// pointer to luma samples
    pub y: *mut u8,
    /// pointer to chroma U samples
    pub u: *mut u8,
    /// pointer to chroma V samples
    pub v: *mut u8,
    /// pointer to alpha samples
    pub a: *mut u8,
    /// luma stride
    pub y_stride: c_int,
    /// chroma U stride
    pub u_stride: c_int,
    /// chroma V stride
    pub v_stride: c_int,
    /// alpha stride
    pub a_stride: c_int,
    /// luma plane size
    pub y_size: usize,
    /// chroma U plane size
    pub u_size: usize,
    /// chroma V planes size
    pub v_size: usize,
    /// alpha-plane size
    pub a_size: usize,
}

/// Output buffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPDecBuffer {
    /// Colorspace.
    pub colorspace: WEBP_CSP_MODE,
    /// Dimension (width).
    pub width: c_int,
    /// Dimension (height).
    pub height: c_int,
    /// If non-zero, 'internal_memory' pointer is not
    /// used. If value is '2' or more, the external
    /// memory is considered 'slow' and multiple
    /// read/write will be avoided.
    pub is_external_memory: c_int,
    /// Nameless union of buffer parameters.
    pub u: __WebPDecBufferUnion,
    /// padding for later use
    pub pad: [u32; 4],
    /// Internally allocated memory (only when
    /// is_external_memory is 0). Should not be used
    /// externally, but accessed via the buffer union.
    #[doc(hidden)]
    pub private_memory: *mut u8,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Clone, Copy)]
pub union __WebPDecBufferUnion {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}

impl std::fmt::Debug for __WebPDecBufferUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("<union>")
    }
}

/// Enumeration of the status codes
#[allow(non_camel_case_types)]
pub type VP8StatusCode = u32;

pub const VP8_STATUS_OK: VP8StatusCode = 0;
pub const VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
pub const VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;

/// Deprecated alpha-less version of WebPIDecGetYUVA(): it will ignore the
/// alpha information (if present). Kept for backward compatibility.
#[allow(non_snake_case)]
#[inline]
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

/// Features gathered from the bitstream
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPBitstreamFeatures {
    /// Width in pixels, as read from the bitstream.
    pub width: c_int,
    /// Height in pixels, as read from the bitstream.
    pub height: c_int,
    /// True if the bitstream contains an alpha channel.
    pub has_alpha: c_int,
    /// True if the bitstream is an animation.
    pub has_animation: c_int,
    /// 0 = undefined (/mixed), 1 = lossy, 2 = lossless
    pub format: c_int,
    /// Unused for now. if true, using incremental decoding is not
    /// recommended.
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub no_incremental_decoding: c_int,
    /// Unused for now. TODO(later)
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub rotate: c_int,
    /// Unused for now. should be 0 for now. TODO(later)
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub uv_sampling: c_int,
    /// padding for later use
    #[cfg(not(feature = "0_5"))]
    #[doc(hidden)]
    pub pad: [u32; 2],
    /// padding for later use
    #[cfg(feature = "0_5")]
    #[doc(hidden)]
    pub pad: [u32; 5],
}

/// Decoding options
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPDecoderOptions {
    /// if true, skip the in-loop filtering
    pub bypass_filtering: c_int,
    /// if true, use faster pointwise upsampler
    pub no_fancy_upsampling: c_int,
    /// if true, cropping is applied _first_
    pub use_cropping: c_int,
    /// left position for cropping.
    /// Will be snapped to even value.
    pub crop_left: c_int,
    /// top position for cropping.
    /// Will be snapped to even value.
    pub crop_top: c_int,
    /// width of the cropping area
    pub crop_width: c_int,
    /// height of the cropping area
    pub crop_height: c_int,
    /// if true, scaling is applied _afterward_
    pub use_scaling: c_int,
    /// final resolution width
    pub scaled_width: c_int,
    /// final resolution height
    pub scaled_height: c_int,
    /// if true, use multi-threaded decoding
    pub use_threads: c_int,
    /// dithering strength (0=Off, 100=full)
    pub dithering_strength: c_int,
    /// if true, flip output vertically
    #[cfg(feature = "0_5")]
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "0_5")))]
    pub flip: c_int,
    /// alpha dithering strength in [0..100]
    #[cfg(feature = "0_5")]
    #[cfg_attr(feature = "__doc_cfg", doc(cfg(feature = "0_5")))]
    pub alpha_dithering_strength: c_int,
    /// Unused for now. forced rotation (to be applied _last_)
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub force_rotation: c_int,
    /// Unused for now. if true, discard enhancement layer
    #[cfg(not(feature = "0_5"))]
    #[deprecated(note = "Removed as of libwebp 0.5.0")]
    pub no_enhancement: c_int,
    /// padding for later use
    #[doc(hidden)]
    pub pad: [u32; 5],
}

/// Main object storing the configuration for advanced decoding.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebPDecoderConfig {
    /// Immutable bitstream features (optional)
    pub input: WebPBitstreamFeatures,
    /// Output buffer (can point to external mem)
    pub output: WebPDecBuffer,
    /// Decoding options
    pub options: WebPDecoderOptions,
}

extern "C" {
    /// Return the decoder's version number, packed in hexadecimal using 8bits for
    /// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
    pub fn WebPGetDecoderVersion() -> c_int;
    /// Retrieve basic header information: width, height.
    /// This function will also validate the header, returning true on success,
    /// false otherwise. '*width' and '*height' are only valid on successful return.
    /// Pointers 'width' and 'height' can be passed NULL if deemed irrelevant.
    /// Note: The following chunk sequences (before the raw VP8/VP8L data) are
    /// considered valid by this function:
    /// RIFF + VP8(L)
    /// RIFF + VP8X + (optional chunks) + VP8(L)
    /// ALPH + VP8 <-- Not a valid WebP format: only allowed for internal purpose.
    /// VP8(L)     <-- Not a valid WebP format: only allowed for internal purpose.
    pub fn WebPGetInfo(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> c_int;
    /// Decodes WebP images pointed to by 'data' and returns RGBA samples, along
    /// with the dimensions in *width and *height. The ordering of samples in
    /// memory is R, G, B, A, R, G, B, A... in scan order (endian-independent).
    /// The returned pointer should be deleted calling WebPFree().
    /// Returns NULL in case of error.
    pub fn WebPDecodeRGBA(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    /// Same as WebPDecodeRGBA, but returning A, R, G, B, A, R, G, B... ordered data.
    pub fn WebPDecodeARGB(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    /// Same as WebPDecodeRGBA, but returning B, G, R, A, B, G, R, A... ordered data.
    pub fn WebPDecodeBGRA(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    /// Same as WebPDecodeRGBA, but returning R, G, B, R, G, B... ordered data.
    /// If the bitstream contains transparency, it is ignored.
    pub fn WebPDecodeRGB(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    /// Same as WebPDecodeRGB, but returning B, G, R, B, G, R... ordered data.
    pub fn WebPDecodeBGR(
        data: *const u8,
        data_size: usize,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut u8;
    /// Decode WebP images pointed to by 'data' to Y'UV format(*). The pointer
    /// returned is the Y samples buffer. Upon return, *u and *v will point to
    /// the U and V chroma data. These U and V buffers need NOT be passed to
    /// WebPFree(), unlike the returned Y luma one. The dimension of the U and V
    /// planes are both (*width + 1) / 2 and (*height + 1)/ 2.
    /// Upon return, the Y buffer has a stride returned as '*stride', while U and V
    /// have a common stride returned as '*uv_stride'.
    /// Return NULL in case of error.
    /// (*) Also named Y'CbCr. See: https://en.wikipedia.org/wiki/YCbCr
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
    // These five functions are variants of the above ones, that decode the image
    // directly into a pre-allocated buffer 'output_buffer'. The maximum storage
    // available in this buffer is indicated by 'output_buffer_size'. If this
    // storage is not sufficient (or an error occurred), NULL is returned.
    // Otherwise, output_buffer is returned, for convenience.
    // The parameter 'output_stride' specifies the distance (in bytes)
    // between scanlines. Hence, output_buffer_size is expected to be at least
    // output_stride x picture-height.
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
    // RGB and BGR variants. Here too the transparency information, if present,
    // will be dropped and ignored.
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
    /// WebPDecodeYUVInto() is a variant of WebPDecodeYUV() that operates directly
    /// into pre-allocated luma/chroma plane buffers. This function requires the
    /// strides to be passed: one for the luma plane and one for each of the
    /// chroma ones. The size of each plane buffer is passed as 'luma_size',
    /// 'u_size' and 'v_size' respectively.
    /// Pointer to the luma plane ('*luma') is returned or NULL if an error occurred
    /// during decoding (or because some buffers were found to be too small).
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
    /// Internal, version-checked, entry point
    #[doc(hidden)]
    pub fn WebPInitDecBufferInternal(_: *mut WebPDecBuffer, _: c_int) -> c_int;
    /// Free any memory associated with the buffer. Must always be called last.
    /// Note: doesn't free the 'buffer' structure itself.
    pub fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
    /// Creates a new incremental decoder with the supplied buffer parameter.
    /// This output_buffer can be passed NULL, in which case a default output buffer
    /// is used (with MODE_RGB). Otherwise, an internal reference to 'output_buffer'
    /// is kept, which means that the lifespan of 'output_buffer' must be larger than
    /// that of the returned WebPIDecoder object.
    /// The supplied 'output_buffer' content MUST NOT be changed between calls to
    /// WebPIAppend() or WebPIUpdate() unless 'output_buffer.is_external_memory' is
    /// not set to 0. In such a case, it is allowed to modify the pointers, size and
    /// stride of output_buffer.u.RGBA or output_buffer.u.YUVA, provided they remain
    /// within valid bounds.
    /// All other fields of WebPDecBuffer MUST remain constant between calls.
    /// Returns NULL if the allocation failed.
    pub fn WebPINewDecoder(output_buffer: *mut WebPDecBuffer) -> *mut WebPIDecoder;
    /// This function allocates and initializes an incremental-decoder object, which
    /// will output the RGB/A samples specified by 'csp' into a preallocated
    /// buffer 'output_buffer'. The size of this buffer is at least
    /// 'output_buffer_size' and the stride (distance in bytes between two scanlines)
    /// is specified by 'output_stride'.
    /// Additionally, output_buffer can be passed NULL in which case the output
    /// buffer will be allocated automatically when the decoding starts. The
    /// colorspace 'csp' is taken into account for allocating this buffer. All other
    /// parameters are ignored.
    /// Returns NULL if the allocation failed, or if some parameters are invalid.
    pub fn WebPINewRGB(
        csp: WEBP_CSP_MODE,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: c_int,
    ) -> *mut WebPIDecoder;
    /// This function allocates and initializes an incremental-decoder object, which
    /// will output the raw luma/chroma samples into a preallocated planes if
    /// supplied. The luma plane is specified by its pointer 'luma', its size
    /// 'luma_size' and its stride 'luma_stride'. Similarly, the chroma-u plane
    /// is specified by the 'u', 'u_size' and 'u_stride' parameters, and the chroma-v
    /// plane by 'v' and 'v_size'. And same for the alpha-plane. The 'a' pointer
    /// can be pass NULL in case one is not interested in the transparency plane.
    /// Conversely, 'luma' can be passed NULL if no preallocated planes are supplied.
    /// In this case, the output buffer will be automatically allocated (using
    /// MODE_YUVA) when decoding starts. All parameters are then ignored.
    /// Returns NULL if the allocation failed or if a parameter is invalid.
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
    /// Deprecated version of the above, without the alpha plane.
    /// Kept for backward compatibility.
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
    /// Deletes the WebPIDecoder object and associated memory. Must always be called
    /// if WebPINewDecoder, WebPINewRGB or WebPINewYUV succeeded.
    pub fn WebPIDelete(idec: *mut WebPIDecoder);
    /// Copies and decodes the next available data. Returns VP8_STATUS_OK when
    /// the image is successfully decoded. Returns VP8_STATUS_SUSPENDED when more
    /// data is expected. Returns error in other cases.
    pub fn WebPIAppend(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
    /// A variant of the above function to be used when data buffer contains
    /// partial data from the beginning. In this case data buffer is not copied
    /// to the internal memory.
    /// Note that the value of the 'data' pointer can change between calls to
    /// WebPIUpdate, for instance when the data buffer is resized to fit larger data.
    pub fn WebPIUpdate(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
    /// Returns the RGB/A image decoded so far. Returns NULL if output params
    /// are not initialized yet. The RGB/A output type corresponds to the colorspace
    /// specified during call to WebPINewDecoder() or WebPINewRGB().
    /// *last_y is the index of last decoded row in raster scan order. Some pointers
    /// (*last_y, *width etc.) can be NULL if corresponding information is not
    /// needed. The values in these pointers are only valid on successful (non-NULL)
    /// return.
    pub fn WebPIDecGetRGB(
        idec: *const WebPIDecoder,
        last_y: *mut c_int,
        width: *mut c_int,
        height: *mut c_int,
        stride: *mut c_int,
    ) -> *mut u8;
    /// Same as above function to get a YUVA image. Returns pointer to the luma
    /// plane or NULL in case of error. If there is no alpha information
    /// the alpha pointer '*a' will be returned NULL.
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
    /// Generic call to retrieve information about the displayable area.
    /// If non NULL, the left/right/width/height pointers are filled with the visible
    /// rectangular area so far.
    /// Returns NULL in case the incremental decoder object is in an invalid state.
    /// Otherwise returns the pointer to the internal representation. This structure
    /// is read-only, tied to WebPIDecoder's lifespan and should not be modified.
    pub fn WebPIDecodedArea(
        idec: *const WebPIDecoder,
        left: *mut c_int,
        top: *mut c_int,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *const WebPDecBuffer;
    /// Internal, version-checked, entry point
    #[doc(hidden)]
    pub fn WebPGetFeaturesInternal(
        _: *const u8,
        _: usize,
        _: *mut WebPBitstreamFeatures,
        _: c_int,
    ) -> VP8StatusCode;
    /// Internal, version-checked, entry point
    #[doc(hidden)]
    pub fn WebPInitDecoderConfigInternal(_: *mut WebPDecoderConfig, _: c_int) -> c_int;
    /// Instantiate a new incremental decoder object with the requested
    /// configuration. The bitstream can be passed using 'data' and 'data_size'
    /// parameter, in which case the features will be parsed and stored into
    /// config->input. Otherwise, 'data' can be NULL and no parsing will occur.
    /// Note that 'config' can be NULL too, in which case a default configuration
    /// is used. If 'config' is not NULL, it must outlive the WebPIDecoder object
    /// as some references to its fields will be used. No internal copy of 'config'
    /// is made.
    /// The return WebPIDecoder object must always be deleted calling WebPIDelete().
    /// Returns NULL in case of error (and config->status will then reflect
    /// the error condition, if available).
    pub fn WebPIDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> *mut WebPIDecoder;
    /// Non-incremental version. This version decodes the full data at once, taking
    /// 'config' into account. Returns decoding status (which should be VP8_STATUS_OK
    /// if the decoding was successful). Note that 'config' cannot be NULL.
    pub fn WebPDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode;
}

/// Initialize the structure as empty. Must be called before any other use.
/// Returns false in case of version mismatch
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPInitDecBuffer(buffer: *mut WebPDecBuffer) -> c_int {
    WebPInitDecBufferInternal(buffer, WEBP_DECODER_ABI_VERSION)
}

/// Retrieve features from the bitstream. The *features structure is filled
/// with information gathered from the bitstream.
/// Returns VP8_STATUS_OK when the features are successfully retrieved. Returns
/// VP8_STATUS_NOT_ENOUGH_DATA when more data is needed to retrieve the
/// features from headers. Returns error in other cases.
/// Note: The following chunk sequences (before the raw VP8/VP8L data) are
/// considered valid by this function:
/// RIFF + VP8(L)
/// RIFF + VP8X + (optional chunks) + VP8(L)
/// ALPH + VP8 <-- Not a valid WebP format: only allowed for internal purpose.
/// VP8(L)     <-- Not a valid WebP format: only allowed for internal purpose.
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPGetFeatures(
    data: *const u8,
    data_size: usize,
    features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    WebPGetFeaturesInternal(data, data_size, features, WEBP_DECODER_ABI_VERSION)
}

/// Initialize the configuration as empty. This function must always be
/// called first, unless WebPGetFeatures() is to be called.
/// Returns false in case of mismatched version.
#[allow(non_snake_case)]
#[inline]
pub unsafe extern "C" fn WebPInitDecoderConfig(config: *mut WebPDecoderConfig) -> c_int {
    WebPInitDecoderConfigInternal(config, WEBP_DECODER_ABI_VERSION)
}
