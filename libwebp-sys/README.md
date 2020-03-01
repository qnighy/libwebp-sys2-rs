# libwebp-sys

This is a raw interface to [the libwebp library](https://developers.google.com/speed/webp/download).

## Minimum Supported Rust Version (MSRV)

Rust 1.31.0

## Features

- `demux` ... enables `libwebpdemux` functions.
- `mux` ... enables `libwebpmux` functions.
- `0_5` ... enables functions introduced in libwebp 0.5.0.
- `0_6` ... enables functions introduced in libwebp 0.6.0.
- `1_1` ... enables functions introduced in libwebp 1.1.0.
- `static` ... statically link against the bundled libwebp.
- `extern-types` ... enables `#![feature(extern_types)]`.

## Linking

If libwebp is found in the system, it links against the library.
Otherwise it builds and links against the bundled libwebp.

In these cases, static link is preferred:

- For musl target.
- When cross-compiling.
- `static` feature is turned on.
- `LIBWEBP_SYS_STATIC` environment variable is set to `1` when building.
