use std::os::raw::*;
use std::ptr;
use std::slice;

use crate::boxed::{WebpBox, WebpYuvBox};
use crate::error::WebPSimpleError;

pub mod boxed;
pub mod error;

#[inline]
unsafe fn wrap_bytes<F>(ptr: *mut u8, get_len: F) -> Result<WebpBox<[u8]>, WebPSimpleError>
where
    F: FnOnce() -> usize,
{
    if !ptr.is_null() {
        let len = get_len();
        Ok(WebpBox::from_raw(slice::from_raw_parts_mut(ptr, len)))
    } else {
        Err(WebPSimpleError)
    }
}

/// Return the decoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[allow(non_snake_case)]
pub fn WebPGetDecoderVersion() -> u32 {
    (unsafe { libwebp_sys::WebPGetDecoderVersion() }) as u32
}

/// Retrieve basic header information: width, height.
/// This function will also validate the header, returning true on success,
/// false otherwise. '*width' and '*height' are only valid on successful return.
/// Pointers 'width' and 'height' can be passed NULL if deemed irrelevant.
#[allow(non_snake_case)]
pub fn WebPGetInfo(data: &[u8]) -> Result<(u32, u32), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result =
        unsafe { libwebp_sys::WebPGetInfo(data.as_ptr(), data.len(), &mut width, &mut height) };
    if result != 0 {
        Ok((width as u32, height as u32))
    } else {
        Err(WebPSimpleError)
    }
}

/// Decodes WebP images pointed to by 'data' and returns RGBA samples, along
/// with the dimensions in *width and *height. The ordering of samples in
/// memory is R, G, B, A, R, G, B, A... in scan order (endian-independent).
/// The returned pointer should be deleted calling WebPFree().
/// Returns NULL in case of error.
#[allow(non_snake_case)]
pub fn WebPDecodeRGBA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result =
        unsafe { libwebp_sys::WebPDecodeRGBA(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGBA, but returning A, R, G, B, A, R, G, B... ordered data.
#[allow(non_snake_case)]
pub fn WebPDecodeARGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result =
        unsafe { libwebp_sys::WebPDecodeARGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGBA, but returning B, G, R, A, B, G, R, A... ordered data.
#[allow(non_snake_case)]
pub fn WebPDecodeBGRA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result =
        unsafe { libwebp_sys::WebPDecodeBGRA(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGBA, but returning R, G, B, R, G, B... ordered data.
/// If the bitstream contains transparency, it is ignored.
#[allow(non_snake_case)]
pub fn WebPDecodeRGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result =
        unsafe { libwebp_sys::WebPDecodeRGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 3) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGB, but returning B, G, R, B, G, R... ordered data.
#[allow(non_snake_case)]
pub fn WebPDecodeBGR(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result =
        unsafe { libwebp_sys::WebPDecodeRGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 3) })?;
    Ok((width as u32, height as u32, buf))
}

/// Decode WebP images pointed to by 'data' to Y'UV format(*). The pointer
/// returned is the Y samples buffer. Upon return, *u and *v will point to
/// the U and V chroma data. These U and V buffers need NOT be passed to
/// WebPFree(), unlike the returned Y luma one. The dimension of the U and V
/// planes are both (*width + 1) / 2 and (*height + 1)/ 2.
/// Upon return, the Y buffer has a stride returned as '*stride', while U and V
/// have a common stride returned as '*uv_stride'.
/// Return NULL in case of error.
/// (*) Also named Y'CbCr. See: http://en.wikipedia.org/wiki/YCbCr
#[allow(non_snake_case)]
pub fn WebPDecodeYUV(data: &[u8]) -> Result<(u32, u32, u32, u32, WebpYuvBox), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut u: *mut u8 = ptr::null_mut();
    let mut v: *mut u8 = ptr::null_mut();
    let mut stride: c_int = 0;
    let mut uv_stride: c_int = 0;
    let result = unsafe {
        libwebp_sys::WebPDecodeYUV(
            data.as_ptr(),
            data.len(),
            &mut width,
            &mut height,
            &mut u,
            &mut v,
            &mut stride,
            &mut uv_stride,
        )
    };
    if !result.is_null() {
        let y_len = height as usize * stride as usize;
        let uv_len = (height as usize + 1) / 2 * uv_stride as usize;
        let buf = unsafe {
            WebpYuvBox::from_raw_yuv(
                slice::from_raw_parts_mut(result, y_len),
                slice::from_raw_parts_mut(u, uv_len),
                slice::from_raw_parts_mut(v, uv_len),
            )
        };
        Ok((
            width as u32,
            height as u32,
            stride as u32,
            uv_stride as u32,
            buf,
        ))
    } else {
        Err(WebPSimpleError)
    }
}

/// Return the encoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[allow(non_snake_case)]
pub fn WebPGetEncoderVersion() -> u32 {
    (unsafe { libwebp_sys::WebPGetEncoderVersion() }) as u32
}

fn encode_size_check(len: usize, width: u32, height: u32, stride: u32, pixelwidth: usize) {
    assert_eq!(
        width as c_int as u32, width,
        "width {} not within c_int",
        width
    );
    assert_eq!(
        height as c_int as u32, height,
        "height {} not within c_int",
        height
    );
    assert_eq!(
        stride as c_int as u32, stride,
        "stride {} not within c_int",
        stride
    );
    let width = width as usize;
    let height = height as usize;
    let equal = if width == 0 {
        len == 0
    } else {
        len % pixelwidth == 0 && len / pixelwidth % width == 0 && len / pixelwidth / width == height
    };
    assert!(
        equal,
        "buffer size mismatch: {} * {} * {} != {}",
        width, height, pixelwidth, len
    );
}

macro_rules! wrap_encoder {
    ($name:ident, $pixelwidth:expr) => {
        #[allow(non_snake_case)]
        pub fn $name(
            rgb: &[u8],
            width: u32,
            height: u32,
            stride: u32,
            quality_factor: f32,
        ) -> Result<WebpBox<[u8]>, WebPSimpleError> {
            encode_size_check(rgb.len(), width, height, stride, $pixelwidth);
            let mut output: *mut u8 = ptr::null_mut();
            let result = unsafe {
                libwebp_sys::$name(
                    rgb.as_ptr(),
                    width as c_int,
                    height as c_int,
                    stride as c_int,
                    quality_factor as c_float,
                    &mut output,
                )
            };
            if result != 0 {
                unsafe { wrap_bytes(output, || result) }
            } else {
                Err(WebPSimpleError)
            }
        }
    };
}

wrap_encoder!(WebPEncodeRGB, 3);
wrap_encoder!(WebPEncodeBGR, 3);
wrap_encoder!(WebPEncodeRGBA, 4);
wrap_encoder!(WebPEncodeBGRA, 4);

macro_rules! wrap_lossless_encoder {
    ($name:ident, $pixelwidth:expr) => {
        #[allow(non_snake_case)]
        pub fn $name(
            rgb: &[u8],
            width: u32,
            height: u32,
            stride: u32,
        ) -> Result<WebpBox<[u8]>, WebPSimpleError> {
            encode_size_check(rgb.len(), width, height, stride, $pixelwidth);
            let mut output: *mut u8 = ptr::null_mut();
            let result = unsafe {
                libwebp_sys::$name(
                    rgb.as_ptr(),
                    width as c_int,
                    height as c_int,
                    stride as c_int,
                    &mut output,
                )
            };
            if result != 0 {
                unsafe { wrap_bytes(output, || result) }
            } else {
                Err(WebPSimpleError)
            }
        }
    };
}

wrap_lossless_encoder!(WebPEncodeLosslessRGB, 3);
wrap_lossless_encoder!(WebPEncodeLosslessBGR, 3);
wrap_lossless_encoder!(WebPEncodeLosslessRGBA, 4);
wrap_lossless_encoder!(WebPEncodeLosslessBGRA, 4);

#[cfg(test)]
mod tests {
    use super::*;

    fn lena() -> Vec<u8> {
        include_bytes!("lena.webp").to_vec()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeRGBA() {
        let (width, height, buf) = WebPDecodeRGBA(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255,
                223, 155, 109, 255, 223, 155, 109, 255,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeARGB() {
        let (width, height, buf) = WebPDecodeARGB(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113,
                255, 223, 155, 109, 255, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeBGRA() {
        let (width, height, buf) = WebPDecodeBGRA(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                113, 158, 226, 255, 113, 158, 226, 255, 113, 158, 226, 255, 113, 158, 226, 255,
                109, 155, 223, 255, 109, 155, 223, 255,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeRGB() {
        let (width, height, buf) = WebPDecodeRGB(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 226, 158, 113, 226, 158, 113, 226, 158, 113, 223, 155, 109, 223,
                155, 109, 223, 155, 109, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeBGR() {
        let (width, height, buf) = WebPDecodeBGR(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 226, 158, 113, 226, 158, 113, 226, 158, 113, 223, 155, 109, 223,
                155, 109, 223, 155, 109, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeYUV() {
        let (width, height, stride, uv_stride, buf) = WebPDecodeYUV(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert!(stride >= 128);
        assert!(uv_stride >= 64);
        assert_eq!(&buf.y()[..6], &[165, 165, 165, 165, 162, 162]);
        assert_eq!(&buf.u()[..6], &[98, 98, 98, 98, 98, 98]);
        assert_eq!(&buf.v()[..6], &[161, 161, 161, 161, 161, 161]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPEncodeRGB() {
        let (width, height, buf) = WebPDecodeRGB(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        WebPEncodeRGB(&buf, width, height, width, 50.0).unwrap();
    }
}
