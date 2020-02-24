use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::os::raw::*;
use std::ptr::NonNull;

pub struct WebpBox<T: ?Sized> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T: ?Sized> WebpBox<T> {
    pub unsafe fn from_raw(raw: *mut T) -> WebpBox<T> {
        Self {
            ptr: NonNull::new_unchecked(raw),
            _marker: PhantomData,
        }
    }

    pub fn into_raw(b: WebpBox<T>) -> *mut T {
        let ptr = b.ptr;
        mem::forget(b);
        ptr.as_ptr()
    }
}

impl<T: ?Sized> Deref for WebpBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for WebpBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> Drop for WebpBox<T> {
    fn drop(&mut self) {
        unsafe {
            WebPFree(self.ptr.as_ptr() as *mut c_void);
        }
    }
}

#[cfg(feature = "0.5")]
use libwebp_sys::WebPFree;

#[cfg(not(feature = "0.5"))]
#[allow(non_snake_case)]
unsafe fn WebPFree(ptr: *mut c_void) {
    extern "C" {
        fn free(ptr: *mut c_void);
    }
    free(ptr);
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self as &T, f)
    }
}

pub struct WebpYuvBox {
    y: NonNull<[u8]>,
    u: NonNull<[u8]>,
    v: NonNull<[u8]>,
}

impl WebpYuvBox {
    pub unsafe fn from_raw_yuv(y: *mut [u8], u: *mut [u8], v: *mut [u8]) -> WebpYuvBox {
        Self {
            y: NonNull::new_unchecked(y),
            u: NonNull::new_unchecked(u),
            v: NonNull::new_unchecked(v),
        }
    }

    pub fn into_raw_yuv(self) -> (*mut [u8], *mut [u8], *mut [u8]) {
        let y = self.y.as_ptr();
        let u = self.u.as_ptr();
        let v = self.v.as_ptr();
        mem::forget(self);
        (y, u, v)
    }

    pub fn y(&self) -> &[u8] {
        unsafe { self.y.as_ref() }
    }
    pub fn y_mut(&mut self) -> &mut [u8] {
        unsafe { self.y.as_mut() }
    }

    pub fn u(&self) -> &[u8] {
        unsafe { self.u.as_ref() }
    }
    pub fn u_mut(&mut self) -> &mut [u8] {
        unsafe { self.u.as_mut() }
    }

    pub fn v(&self) -> &[u8] {
        unsafe { self.v.as_ref() }
    }
    pub fn v_mut(&mut self) -> &mut [u8] {
        unsafe { self.v.as_mut() }
    }

    pub fn yuv(&self) -> (&[u8], &[u8], &[u8]) {
        let y = unsafe { self.y.as_ref() };
        let u = unsafe { self.u.as_ref() };
        let v = unsafe { self.v.as_ref() };
        (y, u, v)
    }
    pub fn yuv_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let y = unsafe { self.y.as_mut() };
        let u = unsafe { self.u.as_mut() };
        let v = unsafe { self.v.as_mut() };
        (y, u, v)
    }

    pub fn into_y(self) -> WebpBox<[u8]> {
        let y = self.y;
        mem::forget(self);
        WebpBox {
            ptr: y,
            _marker: PhantomData,
        }
    }
}

impl Drop for WebpYuvBox {
    fn drop(&mut self) {
        unsafe {
            WebPFree(self.y.as_ptr() as *mut c_void);
        }
    }
}

impl fmt::Debug for WebpYuvBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebpYuvBox")
            .field("y", &self.y())
            .field("u", &self.u())
            .field("v", &self.v())
            .finish()
    }
}
