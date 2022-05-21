use libwebp_sys::{WebPMemoryWriterClear, WebPMemoryWriterInit};
use std::mem;

#[test]
fn test_new_and_delete() {
    unsafe {
        let mut buf = mem::zeroed();
        WebPMemoryWriterInit(&mut buf);
        #[cfg(feature = "0_5")]
        WebPMemoryWriterClear(&mut buf);
    }
}
