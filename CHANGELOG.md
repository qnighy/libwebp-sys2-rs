## Unreleased

- Misc
  - Streamline GitHub Actions workflow for 2024 https://github.com/qnighy/libwebp-sys2-rs/pull/27
  - Migrate to doc_auto_cfg https://github.com/qnighy/libwebp-sys2-rs/pull/28
    - It is only relevant to the generated documentation when the nightly-only `__doc_cfg` feature is enabled.

## 0.1.10

- Changed
  - Add function definitions for v1.4.0 (feature `1_4`) and v1.5.0 (feature `1_5`) https://github.com/qnighy/libwebp-sys2-rs/pull/26
  - Update bundled libwebp from v1.3.2 to v1.5.0 https://github.com/qnighy/libwebp-sys2-rs/pull/26

## 0.1.9

- Changed
  - Update bundled libwebp from 1.3.0 (with CVE-2023-4863 patch) to 1.3.2, which officially ships with the vuln fix https://github.com/qnighy/libwebp-sys2-rs/pull/24
    - See [libwebp's release](https://groups.google.com/a/webmproject.org/g/webp-discuss/c/YhVFA45DVfM/m/hS2jI_KFAwAJ)
- Misc
  - Check in lockfile https://github.com/qnighy/libwebp-sys2-rs/pull/22

## 0.1.8

- Fixed
  - Update bundled libwebp to incorporate the CVE-2023-4863 patch https://github.com/qnighy/libwebp-sys2-rs/pull/21

## 0.1.7

- Fixed
  - Fix regression introduced in 0.1.6. The build was failing if the bundled libwebp (1.3.0) is used and one of the encoding functions are used. https://github.com/qnighy/libwebp-sys2-rs/pull/17

## 0.1.6

- Changed
  - Update bundled libwebp from 1.2.2 to 1.3.0 https://github.com/qnighy/libwebp-sys2-rs/pull/14
- Misc
  - CI maintenance https://github.com/qnighy/libwebp-sys2-rs/pull/15

## 0.1.5

- Fixed
  - Fix build with SSE4.1 support https://github.com/qnighy/libwebp-sys2-rs/pull/13

## 0.1.4

- Fixed
  - Build with bundled libwebp no longer fails without another libwebp installed https://github.com/qnighy/libwebp-sys2-rs/pull/11
- Misc
  - Update cfg-if from 0.1.6 to 1.0.0

## 0.1.3

- Changed
  - Update bundled libwebp from 1.2.0 to 1.2.2 https://github.com/qnighy/libwebp-sys2-rs/commit/dcd7f2a47e4cc6e073526848a75a8979295ed1cf
- Misc
  - add AnimDecoder test https://github.com/qnighy/libwebp-sys2-rs/pull/3
  - Reference libwebp as a submodule instead of committed code https://github.com/qnighy/libwebp-sys2-rs/pull/5
  - Add LICENSE file

## 0.1.2

- Changed
  - Update bundled libwebp from 1.1.0 to 1.2.0 https://github.com/qnighy/libwebp-sys2-rs/pull/2
- Added
  - Support libwebp-1.2.0's `qmin`/`qmax` https://github.com/qnighy/libwebp-sys2-rs/pull/2
- Misc
  - Setup GitHub Actions https://github.com/qnighy/libwebp-sys2-rs/pull/1

## 0.1.1

- Carved out from [qnighy/libwebp-rs](https://github.com/qnighy/libwebp-rs) to [qnighy/libwebp-sys2-rs](https://github.com/qnighy/libwebp-sys2-rs).

## 0.1.0

Initial release.
