// Based on https://github.com/rust-lang/libz-sys/blob/1.0.25/build.rs

use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBWEBP_SYS_STATIC");
    println!("cargo:rerun-if-changed=build.rs");
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    // Don't run pkg-config if we're linking statically (we'll build below) and
    // also don't run pkg-config on macOS/FreeBSD/DragonFly. That'll end up printing
    // `-L /usr/lib` which wreaks havoc with linking to an OpenSSL in /usr/local/lib
    // (Homebrew, Ports, etc.)
    let want_static =
        cfg!(feature = "static") || env::var("LIBWEBP_SYS_STATIC").unwrap_or(String::new()) == "1";
    if !want_static &&
       !target.contains("msvc") && // pkg-config just never works here
       !(host_and_target_contain("apple") ||
         host_and_target_contain("freebsd") ||
         host_and_target_contain("dragonfly"))
    {
        let mut config = pkg_config::Config::new();
        config.cargo_metadata(true);
        if config.probe("libwebp").is_ok() {
            if cfg!(feature = "demux") {
                config.probe("libwebpdemux").unwrap();
            }
            if cfg!(feature = "mux") {
                config.probe("libwebpmux").unwrap();
            }
            return;
        }
    }

    if target.contains("msvc") {
        if try_vcpkg() {
            return;
        }
    }

    // Whitelist a bunch of situations where we build unconditionally.
    //
    // MSVC basically never has it preinstalled, MinGW picks up a bunch of weird
    // paths we don't like, `want_static` may force us, cross compiling almost
    // never has a prebuilt version, and musl is almost always static.
    if target.contains("msvc")
        || target.contains("pc-windows-gnu")
        || want_static
        || target != host
        || target.contains("musl")
    {
        return build_libwebp();
    }

    // If we've gotten this far we're probably a pretty standard platform.
    // Almost all platforms here ship libz by default, but some don't have
    // pkg-config files that we would find above.
    //
    // In any case test if zlib is actually installed and if so we link to it,
    // otherwise continue below to build things.
    if libwebp_installed() {
        println!("cargo:rustc-link-lib=webp");
        if cfg!(feature = "demux") {
            println!("cargo:rustc-link-lib=webpdemux");
        }
        if cfg!(feature = "mux") {
            println!("cargo:rustc-link-lib=webpmux");
        }
        return;
    }

    build_libwebp()
}

fn build_libwebp() {
    // For testing purpose
    if let Ok(value) = std::env::var("__LIBWEBP_SYS_FORBID_BUILD") {
        if value == "1" {
            panic!("__LIBWEBP_SYS_FORBID_BUILD is set to 1");
        }
    }
    cc::Build::new()
        // libwebpdecode_la_SOURCES (src/dec)
        .file("c_src/src/dec/alpha_dec.c")
        .file("c_src/src/dec/buffer_dec.c")
        .file("c_src/src/dec/frame_dec.c")
        .file("c_src/src/dec/idec_dec.c")
        .file("c_src/src/dec/io_dec.c")
        .file("c_src/src/dec/quant_dec.c")
        .file("c_src/src/dec/tree_dec.c")
        .file("c_src/src/dec/vp8_dec.c")
        .file("c_src/src/dec/vp8l_dec.c")
        .file("c_src/src/dec/webp_dec.c")
        // libwebpencode_la_SOURCES (src/enc)
        .file("c_src/src/enc/alpha_enc.c")
        .file("c_src/src/enc/analysis_enc.c")
        .file("c_src/src/enc/backward_references_cost_enc.c")
        .file("c_src/src/enc/backward_references_enc.c")
        .file("c_src/src/enc/config_enc.c")
        .file("c_src/src/enc/cost_enc.c")
        .file("c_src/src/enc/filter_enc.c")
        .file("c_src/src/enc/frame_enc.c")
        .file("c_src/src/enc/histogram_enc.c")
        .file("c_src/src/enc/iterator_enc.c")
        .file("c_src/src/enc/near_lossless_enc.c")
        .file("c_src/src/enc/picture_enc.c")
        .file("c_src/src/enc/picture_csp_enc.c")
        .file("c_src/src/enc/picture_psnr_enc.c")
        .file("c_src/src/enc/picture_rescale_enc.c")
        .file("c_src/src/enc/picture_tools_enc.c")
        .file("c_src/src/enc/predictor_enc.c")
        .file("c_src/src/enc/quant_enc.c")
        .file("c_src/src/enc/syntax_enc.c")
        .file("c_src/src/enc/token_enc.c")
        .file("c_src/src/enc/tree_enc.c")
        .file("c_src/src/enc/vp8l_enc.c")
        .file("c_src/src/enc/webp_enc.c")
        // COMMON_SOURCES (src/dsp)
        .file("c_src/src/dsp/alpha_processing.c")
        .file("c_src/src/dsp/cpu.c")
        .file("c_src/src/dsp/dec.c")
        .file("c_src/src/dsp/dec_clip_tables.c")
        .file("c_src/src/dsp/filters.c")
        .file("c_src/src/dsp/lossless.c")
        .file("c_src/src/dsp/rescaler.c")
        .file("c_src/src/dsp/upsampling.c")
        .file("c_src/src/dsp/yuv.c")
        // ENC_SOURCES (src/dsp)
        .file("c_src/src/dsp/cost.c")
        .file("c_src/src/dsp/enc.c")
        .file("c_src/src/dsp/lossless_enc.c")
        .file("c_src/src/dsp/ssim.c")
        // libwebpdspdecode_sse41_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/alpha_processing_sse41.c")
        .file("c_src/src/dsp/dec_sse41.c")
        .file("c_src/src/dsp/lossless_sse41.c")
        .file("c_src/src/dsp/upsampling_sse41.c")
        .file("c_src/src/dsp/yuv_sse41.c")
        // libwebpdspdecode_sse2_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/alpha_processing_sse2.c")
        .file("c_src/src/dsp/dec_sse2.c")
        .file("c_src/src/dsp/filters_sse2.c")
        .file("c_src/src/dsp/lossless_sse2.c")
        .file("c_src/src/dsp/rescaler_sse2.c")
        .file("c_src/src/dsp/upsampling_sse2.c")
        .file("c_src/src/dsp/yuv_sse2.c")
        // libwebpdspdecode_neon_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/alpha_processing_neon.c")
        .file("c_src/src/dsp/dec_neon.c")
        .file("c_src/src/dsp/filters_neon.c")
        .file("c_src/src/dsp/lossless_neon.c")
        .file("c_src/src/dsp/rescaler_neon.c")
        .file("c_src/src/dsp/upsampling_neon.c")
        .file("c_src/src/dsp/yuv_neon.c")
        // libwebpdspdecode_msa_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/dec_msa.c")
        .file("c_src/src/dsp/filters_msa.c")
        .file("c_src/src/dsp/lossless_msa.c")
        .file("c_src/src/dsp/rescaler_msa.c")
        .file("c_src/src/dsp/upsampling_msa.c")
        // libwebpdspdecode_mips32_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/dec_mips32.c")
        .file("c_src/src/dsp/rescaler_mips32.c")
        .file("c_src/src/dsp/yuv_mips32.c")
        // libwebpdspdecode_mips_dsp_r2_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/alpha_processing_mips_dsp_r2.c")
        .file("c_src/src/dsp/dec_mips_dsp_r2.c")
        .file("c_src/src/dsp/filters_mips_dsp_r2.c")
        .file("c_src/src/dsp/lossless_mips_dsp_r2.c")
        .file("c_src/src/dsp/rescaler_mips_dsp_r2.c")
        .file("c_src/src/dsp/upsampling_mips_dsp_r2.c")
        .file("c_src/src/dsp/yuv_mips_dsp_r2.c")
        // libwebpdsp_sse2_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/cost_sse2.c")
        .file("c_src/src/dsp/enc_sse2.c")
        .file("c_src/src/dsp/lossless_enc_sse2.c")
        .file("c_src/src/dsp/ssim_sse2.c")
        // libwebpdsp_sse41_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/enc_sse41.c")
        .file("c_src/src/dsp/lossless_enc_sse41.c")
        // libwebpdsp_neon_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/cost_neon.c")
        .file("c_src/src/dsp/enc_neon.c")
        .file("c_src/src/dsp/lossless_enc_neon.c")
        // libwebpdsp_msa_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/enc_msa.c")
        .file("c_src/src/dsp/lossless_enc_msa.c")
        // libwebpdsp_mips32_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/cost_mips32.c")
        .file("c_src/src/dsp/enc_mips32.c")
        .file("c_src/src/dsp/lossless_enc_mips32.c")
        // libwebpdsp_mips_dsp_r2_la_SOURCES (src/dsp)
        .file("c_src/src/dsp/cost_mips_dsp_r2.c")
        .file("c_src/src/dsp/enc_mips_dsp_r2.c")
        .file("c_src/src/dsp/lossless_enc_mips_dsp_r2.c")
        // COMMON_SOURCES (sharpyuv)
        .file("c_src/sharpyuv/sharpyuv.c")
        .file("c_src/sharpyuv/sharpyuv_cpu.c")
        .file("c_src/sharpyuv/sharpyuv_csp.c")
        .file("c_src/sharpyuv/sharpyuv_dsp.c")
        .file("c_src/sharpyuv/sharpyuv_gamma.c")
        .file("c_src/sharpyuv/sharpyuv_neon.c")
        .file("c_src/sharpyuv/sharpyuv_sse2.c")
        // COMMON_SOURCES (src/utils)
        .file("c_src/src/utils/bit_reader_utils.c")
        .file("c_src/src/utils/color_cache_utils.c")
        .file("c_src/src/utils/filters_utils.c")
        .file("c_src/src/utils/huffman_utils.c")
        .file("c_src/src/utils/palette.c")
        .file("c_src/src/utils/quant_levels_dec_utils.c")
        .file("c_src/src/utils/rescaler_utils.c")
        .file("c_src/src/utils/random_utils.c")
        .file("c_src/src/utils/thread_utils.c")
        .file("c_src/src/utils/utils.c")
        // ENC_SOURCES (src/utils)
        .file("c_src/src/utils/bit_writer_utils.c")
        .file("c_src/src/utils/huffman_encode_utils.c")
        .file("c_src/src/utils/quant_levels_utils.c")
        .include("c_src")
        .compile("webp");
    if cfg!(feature = "demux") {
        cc::Build::new()
            // libwebpdemux_la_SOURCES (src/demux)
            .file("c_src/src/demux/anim_decode.c")
            .file("c_src/src/demux/demux.c")
            .include("c_src")
            .compile("webpdemux");
    }
    if cfg!(feature = "mux") {
        cc::Build::new()
            // libwebpmux_la_SOURCES (src/mux)
            .file("c_src/src/mux/anim_encode.c")
            .file("c_src/src/mux/muxedit.c")
            .file("c_src/src/mux/muxinternal.c")
            .file("c_src/src/mux/muxread.c")
            .include("c_src")
            .compile("webpmux");
    }
}

#[cfg(not(target_env = "msvc"))]
fn try_vcpkg() -> bool {
    false
}

#[cfg(target_env = "msvc")]
fn try_vcpkg() -> bool {
    // see if there is a vcpkg tree with libwebp installed
    match vcpkg::Config::new()
        .emit_includes(true)
        .lib_name("libwebp")
        .probe("libwebp")
    {
        Ok(_) => true,
        Err(e) => {
            println!("note, vcpkg did not find libwebp: {}", e);
            false
        }
    }
}

fn libwebp_installed() -> bool {
    let compiler = cc::Build::new().get_compiler();
    let mut cmd = Command::new(compiler.path());
    cmd.arg("src/smoke.c")
        .arg("-o")
        .arg("/dev/null")
        .arg("-lwebp");

    println!("running {:?}", cmd);
    if let Ok(status) = cmd.status() {
        if status.success() {
            return true;
        }
    }

    false
}
