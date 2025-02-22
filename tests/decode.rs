use libwebp_sys::{MODE_RGB, WebPIDelete, WebPINewDecoder, WebPInitDecBuffer};
use std::mem;

#[test]
fn test_new_and_delete() {
    unsafe {
        let mut buf = mem::zeroed();
        WebPInitDecBuffer(&mut buf);
        buf.colorspace = MODE_RGB;
        buf.is_external_memory = 0;
        let idec = WebPINewDecoder(&mut buf);
        assert!(!idec.is_null());
        WebPIDelete(idec);
    }
}
