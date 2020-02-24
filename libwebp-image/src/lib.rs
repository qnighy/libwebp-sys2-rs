use std::io::{self, Read, Write};
use std::ops::Deref;

use image::error::{DecodingError, EncodingError, ImageFormatHint};
use image::{
    Bgr, Bgra, ColorType, DynamicImage, ImageBuffer, ImageDecoder, ImageError, ImageResult, Rgb,
    RgbImage, Rgba, RgbaImage,
};
use libwebp::boxed::WebpBox;

#[derive(Debug)]
pub struct WebpReader<R: Read> {
    reader: Reader<R>,
    index: usize,
}

impl<R: Read> WebpReader<R> {
    fn new(reader: Reader<R>) -> Self {
        Self { reader, index: 0 }
    }
}

impl<R: Read> Read for WebpReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let (_, _, _, image_buf) = self.reader.data().unwrap();
        let new_index = (self.index + buf.len()).min(image_buf.len());
        let num_written = new_index - self.index;
        buf[..num_written].copy_from_slice(&image_buf[self.index..new_index]);
        self.index = new_index;
        Ok(num_written)
    }
}

#[derive(Debug)]
pub struct WebpDecoder<R: Read> {
    reader: Reader<R>,
}

#[derive(Debug)]
enum WebpColor {
    RGB,
    RGBA,
}

impl<R: Read> WebpDecoder<R> {
    pub fn new(reader: R) -> ImageResult<Self> {
        Self::new_inner(reader, WebpColor::RGBA)
    }

    pub fn new_rgba(reader: R) -> ImageResult<Self> {
        Self::new_inner(reader, WebpColor::RGBA)
    }

    pub fn new_rgb(reader: R) -> ImageResult<Self> {
        Self::new_inner(reader, WebpColor::RGB)
    }

    fn new_inner(reader: R, colortype: WebpColor) -> ImageResult<Self> {
        let mut reader = Reader::new(reader, colortype);
        reader.read_info()?;
        Ok(Self { reader })
    }
}

impl<'a, R: Read + 'a> ImageDecoder<'a> for WebpDecoder<R> {
    type Reader = WebpReader<R>;

    fn dimensions(&self) -> (u32, u32) {
        self.reader.info().unwrap()
    }
    fn color_type(&self) -> ColorType {
        match self.reader.colortype {
            WebpColor::RGB => ColorType::Rgb8,
            WebpColor::RGBA => ColorType::Rgba8,
        }
    }
    fn into_reader(mut self) -> ImageResult<Self::Reader> {
        self.reader.read_data()?;
        Ok(WebpReader::new(self.reader))
    }
}

const READER_READ_UNIT: usize = 1024;

#[derive(Debug)]
struct Reader<R: Read> {
    reader: R,
    colortype: WebpColor,
    buf: Vec<u8>,
    info: Option<(u32, u32)>,
    data: Option<(u32, u32, u32, WebpBox<[u8]>)>,
    error: bool,
}

impl<R: Read> Reader<R> {
    fn new(reader: R, colortype: WebpColor) -> Self {
        Self {
            reader,
            colortype,
            buf: Vec::new(),
            info: None,
            data: None,
            error: false,
        }
    }

    fn info(&self) -> Option<(u32, u32)> {
        self.info
    }

    fn read_info(&mut self) -> io::Result<()> {
        if self.info.is_some() {
            return Ok(());
        }
        loop {
            let read_len = self.read_into_buf(READER_READ_UNIT)?;
            if let Ok(info) = libwebp::WebPGetInfo(&self.buf) {
                self.info = Some(info);
                return Ok(());
            }
            if read_len == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid webp header",
                ));
            }
        }
    }

    fn data(&self) -> Option<(u32, u32, u32, &[u8])> {
        let (w, h, s, ref buf) = *self.data.as_ref()?;
        Some((w, h, s, buf))
    }

    fn read_data(&mut self) -> io::Result<()> {
        self.read_info()?;
        if self.data.is_some() {
            return Ok(());
        }

        self.reader.read_to_end(&mut self.buf)?;
        let data = match self.colortype {
            WebpColor::RGB => libwebp::WebPDecodeRGB(&self.buf).map(|(w, h, b)| (w, h, w * 3, b)),
            WebpColor::RGBA => libwebp::WebPDecodeRGBA(&self.buf).map(|(w, h, b)| (w, h, w * 4, b)),
        }
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid webp data"))?;
        self.data = Some(data);
        Ok(())
    }

    fn read_into_buf(&mut self, by: usize) -> io::Result<usize> {
        let old_len = self.buf.len();
        self.buf.resize(old_len + by, 0);
        let result = self.reader.read(&mut self.buf[old_len..]);
        self.buf.resize(old_len + result.as_ref().unwrap_or(&0), 0);
        result
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
        .map_err(|_| DecodingError::new(ImageFormatHint::Unknown, "Webp Format Error".to_string()))
        .map_err(ImageError::Decoding)?;
    Ok(ImageBuffer::from_raw(width, height, buf.to_vec()).unwrap())
}

pub fn webp_load_rgb_from_memory(buf: &[u8]) -> ImageResult<RgbImage> {
    let (width, height, buf) = libwebp::WebPDecodeRGB(buf)
        .map_err(|_| DecodingError::new(ImageFormatHint::Unknown, "Webp Format Error".to_string()))
        .map_err(ImageError::Decoding)?;
    Ok(ImageBuffer::from_raw(width, height, buf.to_vec()).unwrap())
}

pub fn webp_write<W: Write>(img: &DynamicImage, w: &mut W) -> ImageResult<()> {
    match img {
        &DynamicImage::ImageRgb8(ref img) => webp_write_rgb(img, w),
        &DynamicImage::ImageRgba8(ref img) => webp_write_rgba(img, w),
        &DynamicImage::ImageBgr8(ref img) => webp_write_bgr(img, w),
        &DynamicImage::ImageBgra8(ref img) => webp_write_bgra(img, w),
        &DynamicImage::ImageLuma8(_) => webp_write_rgb(&img.to_rgb(), w),
        &DynamicImage::ImageLumaA8(_) => webp_write_rgba(&img.to_rgba(), w),
        &DynamicImage::ImageRgb16(_) => webp_write_rgb(&img.to_rgb(), w),
        &DynamicImage::ImageRgba16(_) => webp_write_rgba(&img.to_rgba(), w),
        &DynamicImage::ImageLuma16(_) => webp_write_rgb(&img.to_rgb(), w),
        &DynamicImage::ImageLumaA16(_) => webp_write_rgba(&img.to_rgba(), w),
    }
}

pub fn webp_write_rgba<W: Write, C>(img: &ImageBuffer<Rgba<u8>, C>, w: &mut W) -> ImageResult<()>
where
    C: Deref<Target = [u8]>,
{
    let buf = libwebp::WebPEncodeRGBA(&img, img.width(), img.height(), img.width() * 4, 75.0)
        .map_err(|_| EncodingError::new(ImageFormatHint::Unknown, "Webp Format Error".to_string()))
        .map_err(ImageError::Encoding)?;
    w.write_all(&buf)?;
    Ok(())
}

pub fn webp_write_rgb<W: Write, C>(img: &ImageBuffer<Rgb<u8>, C>, w: &mut W) -> ImageResult<()>
where
    C: Deref<Target = [u8]>,
{
    let buf = libwebp::WebPEncodeRGB(&img, img.width(), img.height(), img.width() * 3, 75.0)
        .map_err(|_| EncodingError::new(ImageFormatHint::Unknown, "Webp Format Error".to_string()))
        .map_err(ImageError::Encoding)?;
    w.write_all(&buf)?;
    Ok(())
}

pub fn webp_write_bgra<W: Write, C>(img: &ImageBuffer<Bgra<u8>, C>, w: &mut W) -> ImageResult<()>
where
    C: Deref<Target = [u8]>,
{
    let buf = libwebp::WebPEncodeBGRA(&img, img.width(), img.height(), img.width() * 4, 75.0)
        .map_err(|_| EncodingError::new(ImageFormatHint::Unknown, "Webp Format Error".to_string()))
        .map_err(ImageError::Encoding)?;
    w.write_all(&buf)?;
    Ok(())
}

pub fn webp_write_bgr<W: Write, C>(img: &ImageBuffer<Bgr<u8>, C>, w: &mut W) -> ImageResult<()>
where
    C: Deref<Target = [u8]>,
{
    let buf = libwebp::WebPEncodeBGR(&img, img.width(), img.height(), img.width() * 3, 75.0)
        .map_err(|_| EncodingError::new(ImageFormatHint::Unknown, "Webp Format Error".to_string()))
        .map_err(ImageError::Encoding)?;
    w.write_all(&buf)?;
    Ok(())
}
