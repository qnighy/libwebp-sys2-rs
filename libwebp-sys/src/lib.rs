#[macro_use]
extern crate cfg_if;

extern crate libc;

pub use crate::decode::*;
pub use crate::demux::*;
pub use crate::encode::*;
pub use crate::mux::*;
pub use crate::mux_types::*;

mod decode;
mod demux;
mod encode;
mod mux;
mod mux_types;

#[allow(non_snake_case)]
pub fn WEBP_ABI_IS_INCOMPATIBLE(a: u16, b: u16) -> bool {
    (a >> 8) != (b >> 8)
}
