#![cfg(feature = "demux")]

use libwebp_sys::{WebPData, WebPDemux, WebPDemuxDelete};

const WEBP_IMAGE: [u8; 94] = [
    0x52, 0x49, 0x46, 0x46, 0x56, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38, 0x20,
    0x4A, 0x00, 0x00, 0x00, 0xD0, 0x01, 0x00, 0x9D, 0x01, 0x2A, 0x03, 0x00, 0x02, 0x00, 0x02, 0x00,
    0x34, 0x25, 0xA8, 0x02, 0x74, 0x01, 0x0E, 0xFE, 0x03, 0x8E, 0x00, 0x00, 0xFE, 0xAD, 0xFF, 0xF1,
    0x5C, 0xB4, 0xF8, 0xED, 0xFF, 0xF0, 0xC0, 0xBA, 0xBF, 0x93, 0x05, 0xEA, 0x0C, 0x9F, 0x93, 0x3F,
    0xE8, 0xC0, 0xBF, 0x3F, 0xFF, 0xA9, 0xBF, 0xFF, 0x24, 0x7B, 0xCB, 0xFF, 0x46, 0x05, 0xF9, 0xFF,
    0xFD, 0x4D, 0xFE, 0x30, 0xE5, 0x86, 0xAA, 0x07, 0x31, 0x23, 0x6F, 0x00, 0x00, 0x00,
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

#[test]
#[cfg(all(feature = "0_5", feature = "demux"))]
fn test_anim_decoder() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::mem;

    use libwebp_sys::{
        WebPAnimDecoderDelete, WebPAnimDecoderGetInfo, WebPAnimDecoderGetNext,
        WebPAnimDecoderHasMoreFrames, WebPAnimDecoderNew, WebPAnimDecoderOptionsInit,
    };

    unsafe {
        let mut buf = Vec::new();
        let len = File::open("./tests/animated.webp")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        let mut options = mem::zeroed();
        assert!(WebPAnimDecoderOptionsInit(&mut options) != 0);

        let data = WebPData {
            bytes: buf.as_ptr(),
            size: len,
        };
        let decoder = WebPAnimDecoderNew(&data, &options);
        assert!(!decoder.is_null());

        let mut info = mem::zeroed();
        assert!(WebPAnimDecoderGetInfo(decoder, &mut info) != 0);

        assert_eq!(info.loop_count, 0);
        assert_eq!(info.frame_count, 10);
        assert_eq!(info.canvas_width, 400);
        assert_eq!(info.canvas_height, 400);

        assert!(WebPAnimDecoderHasMoreFrames(decoder) > 0);

        let mut expected_timestamp = 40;

        while WebPAnimDecoderHasMoreFrames(decoder) > 0 {
            let mut buf = std::ptr::null_mut();
            let buf_ptr: *mut *mut u8 = &mut buf;
            let mut timestamp: i32 = 42;
            assert!(WebPAnimDecoderGetNext(decoder, buf_ptr, &mut timestamp) > 0);

            assert_eq!(timestamp, expected_timestamp);
            expected_timestamp += 40;
        }

        WebPAnimDecoderDelete(decoder);
    }
}
