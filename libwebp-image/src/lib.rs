extern crate image;
extern crate libwebp;

use std::io::{Read, Write};
use std::ops::Deref;

use image::{
    ColorType, DecodingResult, DynamicImage, ImageBuffer, ImageDecoder, ImageError, ImageResult,
    Rgb, RgbImage, Rgba, RgbaImage,
};
use libwebp::WebpBox;

#[derive(Debug)]
pub struct WebpDecoder<R: Read> {
    reader: R,
    colortype: WebpColor,
    data: Option<(u32, u32, u32, WebpBox<[u8]>)>,
    rowidx: u32,
}

#[derive(Debug)]
enum WebpColor {
    Luma,
    RGB,
    RGBA,
}

impl<R: Read> WebpDecoder<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            colortype: WebpColor::RGBA,
            data: None,
            rowidx: 0,
        }
    }

    pub fn new_rgba(reader: R) -> Self {
        Self::new(reader)
    }

    pub fn new_rgb(reader: R) -> Self {
        Self {
            colortype: WebpColor::RGB,
            ..Self::new(reader)
        }
    }

    pub fn new_grayscale(reader: R) -> Self {
        Self {
            colortype: WebpColor::Luma,
            ..Self::new(reader)
        }
    }

    fn get_data(&mut self) -> ImageResult<(u32, u32, u32, &[u8])> {
        if self.data.is_none() {
            let mut buf = Vec::new();
            self.reader.read_to_end(&mut buf)?;
            let data = match self.colortype {
                WebpColor::Luma => {
                    libwebp::WebPDecodeYUV(&buf).map(|(w, h, s, _, b)| (w, h, s, b.into_y()))
                }
                WebpColor::RGB => libwebp::WebPDecodeRGB(&buf).map(|(w, h, b)| (w, h, w * 3, b)),
                WebpColor::RGBA => libwebp::WebPDecodeRGBA(&buf).map(|(w, h, b)| (w, h, w * 4, b)),
            }
            .map_err(|_| ImageError::FormatError("Webp Format Error".to_string()))?;
            self.data = Some(data);
        }
        let &(width, height, stride, ref buf) = self.data.as_ref().unwrap();
        Ok((width, height, stride, buf))
    }
}

impl<R: Read> ImageDecoder for WebpDecoder<R> {
    fn dimensions(&mut self) -> ImageResult<(u32, u32)> {
        let (width, height, _, _) = self.get_data()?;
        Ok((width, height))
    }
    fn colortype(&mut self) -> ImageResult<ColorType> {
        Ok(match self.colortype {
            WebpColor::Luma => ColorType::Gray(8),
            WebpColor::RGB => ColorType::RGB(8),
            WebpColor::RGBA => ColorType::RGBA(8),
        })
    }
    fn row_len(&mut self) -> ImageResult<usize> {
        let (_, _, stride, _) = self.get_data()?;
        Ok(stride as usize)
    }
    fn read_scanline(&mut self, writebuf: &mut [u8]) -> ImageResult<u32> {
        let rowidx = self.rowidx;
        self.rowidx += 1;

        let (_, _, stride, buf) = self.get_data()?;
        let row_len = stride as usize;
        writebuf.clone_from_slice(&buf[row_len * rowidx as usize..row_len * (rowidx as usize + 1)]);
        Ok(rowidx)
    }
    fn read_image(&mut self) -> ImageResult<DecodingResult> {
        let (_, _, _, buf) = self.get_data()?;
        Ok(DecodingResult::U8(buf.to_vec()))
    }
}

pub fn webp_load<R: Read>(r: R) -> ImageResult<DynamicImage> {
    Ok(DynamicImage::ImageRgba8(webp_load_rgba(r)?))
}

pub fn webp_load_rgba<R: Read>(mut r: R) -> ImageResult<RgbaImage> {
    let mut buf = Vec::new();
    r.read_exact(&mut buf)?;
    webp_load_rgba_from_memory(&buf)
}

pub fn webp_load_rgb<R: Read>(mut r: R) -> ImageResult<RgbImage> {
    let mut buf = Vec::new();
    r.read_exact(&mut buf)?;
    webp_load_rgb_from_memory(&buf)
}

pub fn webp_load_from_memory(buf: &[u8]) -> ImageResult<DynamicImage> {
    Ok(DynamicImage::ImageRgba8(webp_load_rgba_from_memory(buf)?))
}

pub fn webp_load_rgba_from_memory(buf: &[u8]) -> ImageResult<RgbaImage> {
    let (width, height, buf) = libwebp::WebPDecodeRGBA(buf)
        .map_err(|_| ImageError::FormatError("Webp Format Error".to_string()))?;
    Ok(ImageBuffer::from_raw(width, height, buf.to_vec()).unwrap())
}

pub fn webp_load_rgb_from_memory(buf: &[u8]) -> ImageResult<RgbImage> {
    let (width, height, buf) = libwebp::WebPDecodeRGB(buf)
        .map_err(|_| ImageError::FormatError("Webp Format Error".to_string()))?;
    Ok(ImageBuffer::from_raw(width, height, buf.to_vec()).unwrap())
}

pub fn webp_write<W: Write>(img: &DynamicImage, w: &mut W) -> ImageResult<()> {
    match img {
        &DynamicImage::ImageRgb8(ref img) => webp_write_rgb(img, w),
        &DynamicImage::ImageRgba8(ref img) => webp_write_rgba(img, w),
        &DynamicImage::ImageLuma8(_) => webp_write_rgb(&img.to_rgb(), w),
        &DynamicImage::ImageLumaA8(_) => webp_write_rgba(&img.to_rgba(), w),
    }
}

pub fn webp_write_rgba<W: Write, C>(img: &ImageBuffer<Rgba<u8>, C>, w: &mut W) -> ImageResult<()>
where
    C: Deref<Target = [u8]>,
{
    let buf = libwebp::WebPEncodeRGBA(&img, img.width(), img.height(), img.width() * 4, 75.0)
        .map_err(|_| ImageError::FormatError("Webp Format Error".to_string()))?;
    w.write_all(&buf)?;
    Ok(())
}

pub fn webp_write_rgb<W: Write, C>(img: &ImageBuffer<Rgb<u8>, C>, w: &mut W) -> ImageResult<()>
where
    C: Deref<Target = [u8]>,
{
    let buf = libwebp::WebPEncodeRGB(&img, img.width(), img.height(), img.width() * 3, 75.0)
        .map_err(|_| ImageError::FormatError("Webp Format Error".to_string()))?;
    w.write_all(&buf)?;
    Ok(())
}
