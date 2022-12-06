#[allow(unused_macros)]
macro_rules! warn {
    ($fmt:literal $(,$arg:expr)*) => {
        println!(concat!("cargo-warning=", $fmt) $(,$arg)*)
    };
}

pub fn main() {
    let mut cc_builder = cc::Build::new();
    cc_builder.cpp(false).define("MINIAUDIO_IMPLEMENTATION", "");

    // This removes an annoying unused function warning from miniaudio.
    cc_builder.flag_if_supported("-Wno-unused-function");

    apply_flags(&mut cc_builder);
    apply_definitions(|name, value| {
        cc_builder.define(name, value);
    });

    cc_builder
        .include("./miniaudio")
        .file("./miniaudio-wrapper.c");

    if cfg!(feature = "ma-enable-vorbis") {
        cc_builder.file("./miniaudio/extras/stb_vorbis.c");
    }

    cc_builder.compile("libminiaudio");

    emit_supported_features();

    #[cfg(feature = "bindgen")]
    {
        generate_bindings();
    }

    #[cfg(not(feature = "bindgen"))]
    {
        check_pregenerated_bindings();
    }

    // only rebuild if these files are changed.
    println!("cargo:rerun-if-changed=./miniaudio/miniaudio.h");
    println!("cargo:rerun-if-changed=./miniaudio-wrapper.c");
    println!("cargo:rerun-if-env-changed=CC");
}

#[cfg(not(feature = "bindgen"))]
fn check_pregenerated_bindings() {
    use std::path::Path;

    warn!("using pre-generated bindings - these are not guaranteed to always work (!!!)");

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let subdir = format!("{}-{}", os, arch);
    let bindings_dir = Path::new("bindings").join(subdir);

    if !bindings_dir.is_dir() {
        panic!(
            "bindings for {}-{} do not exist in `{}`, please enable the `bindgen` feature instead",
            os,
            arch,
            bindings_dir.display()
        );
    }

    let bindings_file = if cfg!(feature = "ma-enable-vorbis") {
        bindings_dir.join("bindings-with-vorbis.rs").canonicalize()
    } else {
        bindings_dir.join("bindings.rs").canonicalize()
    }
    .expect("failed to canonicalize bindings path, please enable the `bindgen` feature instead");

    if let Some(bindings_file_str) = bindings_file.to_str() {
        println!(
            "cargo:rustc-env=MINIAUDIO_SYS_BINDINGS_FILE={}",
            bindings_file_str
        );
    } else {
        panic!(
            "bindings file path ({}) is not valid UTF-8, please use `bindgen` feature instead",
            bindings_file.display()
        );
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    use std::path::PathBuf;

    let header = if cfg!(feature = "ma-enable-vorbis") {
        "./bindings-with-vorbis.h"
    } else {
        "./bindings.h"
    };

    let bindings = bindgen::Builder::default()
        // Make sure to only whitelist miniaudio's API.
        .allowlist_type("ma_.*")
        .allowlist_function("ma_.*")
        .allowlist_var("(ma|MA)_.*")
        // We just use one big generated header created by concatenating what we need.
        .header(header)
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .layout_tests(true)
        .derive_copy(true)
        .impl_debug(true)
        .prepend_enum_name(false)
        .rust_target(bindgen::RustTarget::Stable_1_36)
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("failed to write bindings");

    println!(
        "cargo:rustc-env=MINIAUDIO_SYS_BINDINGS_FILE={}",
        out_path.display()
    );
}

/// Calls the given closure with the macro definitions that map to features in miniaudio that are enabled
/// through Rust.
fn apply_definitions<F: FnMut(&str, &str)>(mut define: F) {
    if cfg!(feature = "ma-no-flac") {
        define("MA_NO_FLAC", "1");
    }

    if cfg!(feature = "ma-no-mp3") {
        define("MA_NO_MP3", "1");
    }

    if cfg!(feature = "ma-no-wav") {
        define("MA_NO_WAV", "1");
    }

    if cfg!(feature = "ma-no-wasapi") {
        define("MA_NO_WASAPI", "1");
    }

    if cfg!(feature = "ma-no-dsound") {
        define("MA_NO_DSOUND", "1");
    }

    if cfg!(feature = "ma-no-winmm") {
        define("MA_NO_WINMM", "1");
    }
    if cfg!(feature = "ma-no-alsa") {
        define("MA_NO_ALSA", "1");
    }
    if cfg!(feature = "ma-no-pulseaudio") {
        define("MA_NO_PULSEAUDIO", "1");
    }
    if cfg!(feature = "ma-no-jack") {
        define("MA_NO_JACK", "1");
    }
    if cfg!(feature = "ma-no-coreaudio") {
        define("MA_NO_COREAUDIO", "1");
    }
    if cfg!(feature = "ma-no-sndio") {
        define("MA_NO_SNDIO", "1");
    }
    if cfg!(feature = "ma-no-audio4") {
        define("MA_NO_AUDIO4", "1");
    }
    if cfg!(feature = "ma-no-oss") {
        define("MA_NO_OSS", "1");
    }
    if cfg!(feature = "ma-no-aaudio") {
        define("MA_NO_AAUDIO", "1");
    }
    if cfg!(feature = "ma-no-opensl") {
        define("MA_NO_OPENSL", "1");
    }
    if cfg!(feature = "ma-no-webaudio") {
        define("MA_NO_WEBAUDIO", "1");
    }
    if cfg!(feature = "ma-no-null") {
        define("MA_NO_NULL", "1");
    }
    if cfg!(feature = "ma-no-decoding") {
        define("MA_NO_DECODING", "1");
    }
    if cfg!(feature = "ma-no-device-io") {
        define("MA_NO_DEVICE_IO", "1");
    }
    if cfg!(feature = "ma-no-stdio") {
        define("MA_NO_STDIO", "1");
    }
    if cfg!(feature = "ma-no-sse2") {
        define("MA_NO_SSE2", "1");
    }
    if cfg!(feature = "ma-no-avx2") {
        define("MA_NO_AVX2", "1");
    }
    if cfg!(feature = "ma-no-avx512") {
        define("MA_NO_AVX512", "1");
    }
    if cfg!(feature = "ma-no-neon") {
        define("MA_NO_NEON", "1");
    }

    if cfg!(feature = "ma-debug-output") {
        define("MA_DEBUG_OUTPUT", "1");
    }

    let mut log_level: Option<&'static str> = None;
    const LOG_LEVEL_VERBOSE: &'static str = "4";
    const LOG_LEVEL_INFO: &'static str = "3";
    const LOG_LEVEL_WARNING: &'static str = "2";
    const LOG_LEVEL_ERROR: &'static str = "1";

    if cfg!(feature = "ma-log-level-error") {
        log_level = Some(LOG_LEVEL_ERROR);
    }
    if cfg!(feature = "ma-log-level-warning") {
        log_level = Some(LOG_LEVEL_WARNING);
    }
    if cfg!(feature = "ma-log-level-info") {
        log_level = Some(LOG_LEVEL_INFO);
    }
    if cfg!(feature = "ma-log-level-verbose") {
        log_level = Some(LOG_LEVEL_VERBOSE);
    }

    if let Some(level) = log_level {
        define("MA_LOG_LEVEL", level);
    }
}

/// Enables SIMD flags for CC based on enabled Rust features.
#[allow(clippy::logic_bug)]
fn apply_flags(b: &mut cc::Build) {
    if cfg!(target_feature = "sse2") && !(cfg!(feature = "ma-no-sse2")) {
        b.flag_if_supported("-msse2");
    }

    if cfg!(target_feature = "avx2") && !(cfg!(feature = "ma-no-avx2")) {
        b.flag_if_supported("-mavx2");
    }

    if cfg!(target_feature = "avx512") && !(cfg!(feature = "ma-no-avx512")) {
        b.flag_if_supported("-mavx512");
    }

    if cfg!(target_feature = "neon") && !(cfg!(feature = "ma-no-neon")) {
        b.flag_if_supported("-mneon");
    }
}

/// Emits features that map to miniaudio platform feature macros.
#[allow(clippy::logic_bug)]
fn emit_supported_features() {
    let emit_feat = |feature: &'static str| {
        println!("cargo:rustc-cfg=feature=\"{}\"", feature);
    };

    let emit_feat_cond = |feature: &'static str, cond: bool| {
        if cond {
            emit_feat(feature);
        }
    };

    let ma_macos = cfg!(target_os = "macos");
    let ma_ios = cfg!(target_os = "ios");
    let ma_apple = ma_macos | ma_ios;
    let ma_win32 = cfg!(target_family = "windows");
    let ma_win32_desktop = ma_win32; // FIXME for now I just assume they are the same.
    let ma_unix = cfg!(target_family = "unix") && !ma_apple; // MacOS/iOS does not define __unix__ so doesn't define MA_UNIX in miniaudio.
    let ma_android = cfg!(target_os = "android");
    let ma_linux = ma_android | cfg!(target_os = "linux");
    let ma_openbsd = cfg!(target_os = "openbsd");
    let ma_freebsd = cfg!(target_os = "freebsd");
    let ma_netbsd = cfg!(target_os = "netbsd");
    let ma_dragonfly = cfg!(target_os = "dragonfly");
    let ma_bsd = ma_openbsd | ma_freebsd | ma_netbsd | ma_dragonfly;
    let ma_emscripten = cfg!(target_os = "emscripten");

    // #FIXME This is probably not correct but it's not a big deal atm.
    let ma_posix = !ma_win32;

    emit_feat_cond("ma-win32", ma_win32);
    emit_feat_cond("ma-win32-desktop", ma_win32_desktop);
    emit_feat_cond("ma-unix", ma_unix);
    emit_feat_cond("ma-android", ma_android);
    emit_feat_cond("ma-linux", ma_linux);
    emit_feat_cond("ma-openbsd", ma_openbsd);
    emit_feat_cond("ma-freebsd", ma_freebsd);
    emit_feat_cond("ma-netbsd", ma_netbsd);
    emit_feat_cond("ma-dragonfly", ma_dragonfly);
    emit_feat_cond("ma-bsd", ma_bsd);
    emit_feat_cond("ma-emscripten", ma_emscripten);
    emit_feat_cond("ma-macos", ma_macos);
    emit_feat_cond("ma-ios", ma_ios);
    emit_feat_cond("ma-apple", ma_apple);
    emit_feat_cond("ma-posix", ma_posix);

    let mut support_wasapi = false;
    let mut support_dsound = false;
    let mut support_winmm = false;
    let mut support_alsa = false;
    let mut support_jack = false;
    let mut support_pulseaudio = false;
    let mut support_aaudio = false;
    let mut support_opensl = false;
    let mut support_sndio = false;
    let mut support_audio4 = false;
    let mut support_oss = false;
    let mut support_coreaudio = false;
    let mut support_webaudio = false;
    let mut support_null = false;

    if !cfg!(target_feature = "no-device-io") {
        if ma_win32 {
            support_wasapi = true;
            if ma_win32_desktop {
                support_dsound = true;
                support_winmm = true;
                support_jack = true;
            }
        }

        if ma_unix {
            if ma_linux {
                // ALSA is not supported on Android.
                if !ma_android {
                    support_alsa = true;
                }
            }

            if !ma_bsd && !ma_android && !ma_emscripten {
                support_pulseaudio = true;
                support_jack = true;
            }

            if ma_android {
                support_aaudio = true;
                support_opensl = true;
            }

            // FIXME change this to ma_bsd whenever miniaudio decides to do it as well.
            if ma_openbsd {
                // SNDIO is only supported on OpenBSD for now. May be expanded later if there is
                // demand.
                support_sndio = true;
            }

            if ma_netbsd || ma_openbsd {
                // Only support audio(4) on platforms with known support.
                support_audio4 = true;
            }

            if ma_freebsd || ma_dragonfly {
                // Only support OSS on specific platforms with known support.
                support_oss = true;
            }
        }

        if ma_apple {
            support_coreaudio = true;
        }

        if ma_emscripten {
            support_webaudio = true;
        }

        // Explicitly disable the null backend for Emscripten because it uses a background thread
        // which is not properly supported right now.
        if !ma_emscripten {
            support_null = true;
        }
    }

    //
    // EMIT SUPPORT FLAGS:
    //
    emit_feat_cond("ma-support-wasapi", support_wasapi);
    emit_feat_cond("ma-support-dsound", support_dsound);
    emit_feat_cond("ma-support-winmm", support_winmm);
    emit_feat_cond("ma-support-alsa", support_alsa);
    emit_feat_cond("ma-support-pulseaudio", support_pulseaudio);
    emit_feat_cond("ma-support-jack", support_jack);
    emit_feat_cond("ma-support-aaudio", support_aaudio);
    emit_feat_cond("ma-support-opensl", support_opensl);
    emit_feat_cond("ma-support-sndio", support_sndio);
    emit_feat_cond("ma-support-audio4", support_audio4);
    emit_feat_cond("ma-support-oss", support_oss);
    emit_feat_cond("ma-support-coreaudio", support_coreaudio);
    emit_feat_cond("ma-support-webaudio", support_webaudio);
    emit_feat_cond("ma-support-null", support_null);

    //
    // EMIT ENABLE FLAGS:
    //
    emit_feat_cond(
        "ma-enable-wasapi",
        !cfg!(feature = "ma-no-wasapi") && support_wasapi,
    );
    emit_feat_cond(
        "ma-enable-dsound",
        !cfg!(feature = "ma-no-dsound") && support_dsound,
    );
    emit_feat_cond(
        "ma-enable-winmm",
        !cfg!(feature = "ma-no-winmm") && support_winmm,
    );
    emit_feat_cond(
        "ma-enable-jack",
        !cfg!(feature = "ma-no-jack") && support_jack,
    );
    emit_feat_cond(
        "ma-enable-alsa",
        !cfg!(feature = "ma-no-alsa") && support_alsa,
    );
    emit_feat_cond(
        "ma-enable-pulseaudio",
        !cfg!(feature = "ma-no-pulseaudio") && support_pulseaudio,
    );
    emit_feat_cond(
        "ma-enable-aaudio",
        !cfg!(feature = "ma-no-aaudio") && support_aaudio,
    );
    emit_feat_cond(
        "ma-enable-opensl",
        !cfg!(feature = "ma-no-opensl") && support_opensl,
    );
    emit_feat_cond(
        "ma-enable-sndio",
        !cfg!(feature = "ma-no-sndio") && support_sndio,
    );
    emit_feat_cond(
        "ma-enable-audio4",
        !cfg!(feature = "ma-no-audio4") && support_audio4,
    );
    emit_feat_cond("ma-enable-oss", !cfg!(feature = "ma-no-oss") && support_oss);
    emit_feat_cond(
        "ma-enable-coreaudio",
        !cfg!(feature = "ma-no-coreaudio") && support_coreaudio,
    );
    emit_feat_cond(
        "ma-enable-webaudio",
        !cfg!(feature = "ma-no-webaudio") && support_webaudio,
    );
    emit_feat_cond(
        "ma-enable-null",
        !cfg!(feature = "ma-no-null") && support_null,
    );
}
