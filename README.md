# libwebp

This is a binding to [the libwebp library](https://developers.google.com/speed/webp/download).

## Minimum Supported Rust Version (MSRV)

Rust 1.31.0

## Features

- `demux` ... enables `libwebpdemux` functions.
- `mux` ... enables `libwebpmux` functions.
- `0.5` ... enables functions introduced in libwebp 0.5.0.
- `0.6` ... enables functions introduced in libwebp 0.6.0.
- `1.1` ... enables functions introduced in libwebp 1.1.0.
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

## Completeness

- `types.h`
  - [ ] `WebPMalloc`
  - [x] `WebPFree`
- `decode.h`
  - [x] `WebPGetDecoderVersion`
  - [x] `WebPGetInfo`
  - [x] `WebPDecode*`
  - [x] `WebPDecodeYUV`
  - [ ] `WebPDecode*Into`
  - [ ] `WebPDecodeYUVInto`
  - [ ] `WebPFreeDecBuffer`
  - [ ] `WEBP_CSP_MODE`
  - [ ] `WebPIsPremultipliedMode`
  - [ ] `WebPIsAlphaMode`
  - [ ] `WebPIsRGBMode`
  - [ ] `WebPRGBABuffer`
  - [ ] `WebPYUVABuffer`
  - [ ] `WebPDecBuffer`
  - [ ] `WebPInitDecBuffer`
  - [ ] `WebPFreeDecBuffer`
  - [ ] `VP8StatusCode`
  - [ ] `WebPIDecoder`
  - [ ] `WebPINewDecoder`
  - [ ] `WebPINewRGB`
  - [ ] `WebPINewYUVA`
  - [ ] `WEBPINewYUV`
  - [ ] `WebPIDelete`
  - [ ] `WebPIAppend`
  - [ ] `WebPIUpdate`
  - [ ] `WebPIDecGetRGB`
  - [ ] `WebPIDecGetYUVA`
  - [ ] `WebPIDecGetYUV`
  - [ ] `WebPIDecodedArea`
  - [ ] `WebPBitstreamFeatures`
  - [ ] `WebPGetFeatures`
  - [ ] `WebPDecoderOptions`
  - [ ] `WebPDecoderConfig`
  - [ ] `WebPInitDecoderConfig`
  - [ ] `WebPIDecode`
  - [ ] `WebPDecode`
- `encode.h`
  - [x] `WebPGetEncoderVersion`
  - [x] `WebPEncode*`
  - [x] `WebPEncodeLossless*`
  - [ ] `WebPImageHint`
  - [ ] `WebPConfig`
  - [ ] `WebPPreset`
  - [ ] `WebPConfigInit`
  - [ ] `WebPConfigPreset`
  - [ ] `WebPConfigLosslessPreset`
  - [ ] `WebPValidateConfig`
  - [ ] `WebPAuxStats`
  - [ ] `WebPWriterFunction`
  - [ ] `WebPMemoryWriter`
  - [ ] `WebPMemoryWriterInit`
  - [ ] `WebPMemoryWriterClear`
  - [ ] `WebPMemoryWrite`
  - [ ] `WebPProgressHook`
  - [ ] `WebPEncCSP`
  - [ ] `WebPEncodingError`
  - [ ] `WEBP_MAX_DIMENSION`
  - [ ] `WebPPicture`
  - [ ] `WebPPictureInit`
  - [ ] `WebPPictureAlloc`
  - [ ] `WebPPictureFree`
  - [ ] `WebPPictureCopy`
  - [ ] `WebPPlaneDistortion`
  - [ ] `WebPPictureDistortion`
  - [ ] `WebPPictureCrop`
  - [ ] `WebPPictureView`
  - [ ] `WebPPictureIsView`
  - [ ] `WebPPictureRescale`
  - [ ] `WebPPictureImportRGB`
  - [ ] `WebPPictureImportRGBA`
  - [ ] `WebPPictureImportRGBX`
  - [ ] `WebPPictureImportBGR`
  - [ ] `WebPPictureImportBGRA`
  - [ ] `WebPPictureImportBGRX`
  - [ ] `WebPPictureARGBToYUVA`
  - [ ] `WebPPictureARGBToYUVADithered`
  - [ ] `WebPPictureSharpARGBToYUVA`
  - [ ] `WebPPictureSmartARGBToYUVA`
  - [ ] `WebPPictureYUVAToARGB`
  - [ ] `WebPCleanupTransparentArea`
  - [ ] `WebPPictureHasTransparency`
  - [ ] `WebPBlendAlpha`
  - [ ] `WebPEncode`
- `mux_types.h`
  - Not at all
- `demux.h`
  - Not at all
- `mux.h`
  - Not at all


