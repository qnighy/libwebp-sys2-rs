#![cfg(feature = "mux")]

use libwebp_sys::{WebPMuxDelete, WebPMuxNew};

#[test]
fn test_new_and_delete() {
    unsafe {
        let ptr = WebPMuxNew();
        assert!(!ptr.is_null());
        WebPMuxDelete(ptr);
    }
}
