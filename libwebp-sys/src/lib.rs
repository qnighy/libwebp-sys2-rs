#![cfg_attr(feature = "__doc_cfg", feature(doc_cfg))]

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
