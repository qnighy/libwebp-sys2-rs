#[cfg(feature = "1_1")]
#[test]
fn test_malloc() {
    use libwebp_sys::{WebPFree, WebPMalloc};

    unsafe {
        let ptr = WebPMalloc(12);
        assert!(!ptr.is_null());
        WebPFree(ptr);
    }
}
