#![cfg_attr(feature = "__doc_cfg", feature(doc_cfg))]
#![cfg_attr(feature = "__doc_cfg", feature(doc_auto_cfg))]
#![cfg_attr(feature = "extern-types", feature(extern_types))]

#[macro_use]
extern crate cfg_if;

pub use crate::decode::*;
#[cfg(feature = "demux")]
pub use crate::demux::*;
pub use crate::encode::*;
#[cfg(feature = "mux")]
pub use crate::mux::*;
#[cfg(any(feature = "mux", feature = "demux"))]
pub use crate::mux_types::*;
pub use crate::types::*;

mod decode;
#[cfg(feature = "demux")]
mod demux;
mod encode;
#[cfg(feature = "mux")]
mod mux;
#[cfg(any(feature = "mux", feature = "demux"))]
mod mux_types;
mod types;

#[allow(unused)]
fn ensure_rust_1_85() {
    let _ = 0_u32.midpoint(2);
}
