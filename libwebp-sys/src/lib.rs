#[macro_use]
extern crate cfg_if;

extern crate libc;

pub use decode::*;
pub use demux::*;
pub use encode::*;
pub use mux::*;
pub use mux_types::*;

mod decode;
mod demux;
mod encode;
mod mux;
mod mux_types;

#[allow(non_snake_case)]
pub fn WEBP_ABI_IS_INCOMPATIBLE(a: u16, b: u16) -> bool {
    (a >> 8) != (b >> 8)
}
