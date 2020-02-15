#[macro_use]
extern crate cfg_if;

pub use crate::decode::*;
#[cfg(feature = "demux")]
pub use crate::demux::*;
pub use crate::encode::*;
pub use crate::format_constants::*;
#[cfg(feature = "mux")]
pub use crate::mux::*;
#[cfg(any(feature = "mux", feature = "demux"))]
pub use crate::mux_types::*;
pub use crate::types::*;

mod decode;
#[cfg(feature = "demux")]
mod demux;
mod encode;
mod format_constants;
#[cfg(feature = "mux")]
mod mux;
#[cfg(any(feature = "mux", feature = "demux"))]
mod mux_types;
mod types;
