extern crate cc;

pub fn main() {
    let mut cc_builder = cc::Build::new();
    cc_builder.cpp(false).define("MINIAUDIO_IMPLEMENTATION", "");
    apply_flags(&mut cc_builder);
    apply_definitions(&mut cc_builder);
    cc_builder
        .include("./miniaudio")
        .file("./miniaudio-wrapper.c")
        .compile("libminiaudio");

    // only rebuild if these files are changed.
    println!("cargo:rerun-if-changed=./miniaudio/miniaudio.h");
    println!("cargo:rerun-if-changed=./miniaudio-wrapper.c");
    println!("cargo:rerun-if-env-changed=CC");
}

fn apply_definitions(b: &mut cc::Build) {
    if cfg!(feature = "no-wasapi") {
        b.define("MA_NO_WASAPI", "1");
    }

    if cfg!(feature = "no-dsound") {
        b.define("MA_NO_DSOUND", "1");
    }

    if cfg!(feature = "no-winmm") {
        b.define("MA_NO_WINMM", "1");
    }
    if cfg!(feature = "no-alsa") {
        b.define("MA_NO_ALSA", "1");
    }
    if cfg!(feature = "no-pulseaudio") {
        b.define("MA_NO_PULSEAUDIO", "1");
    }
    if cfg!(feature = "no-jack") {
        b.define("MA_NO_JACK", "1");
    }
    if cfg!(feature = "no-coreaudio") {
        b.define("MA_NO_COREAUDIO", "1");
    }
    if cfg!(feature = "no-sndio") {
        b.define("MA_NO_SNDIO", "1");
    }
    if cfg!(feature = "no-audio4") {
        b.define("MA_NO_AUDIO4", "1");
    }
    if cfg!(feature = "no-oss") {
        b.define("MA_NO_OSS", "1");
    }
    if cfg!(feature = "no-aaudio") {
        b.define("MA_NO_AAUDIO", "1");
    }
    if cfg!(feature = "no-opensl") {
        b.define("MA_NO_OPENSL", "1");
    }
    if cfg!(feature = "no-webaudio") {
        b.define("MA_NO_WEBAUDIO", "1");
    }
    if cfg!(feature = "no-null") {
        b.define("MA_NO_NULL", "1");
    }
    if cfg!(feature = "no-decoding") {
        b.define("MA_NO_DECODING", "1");
    }
    if cfg!(feature = "no-device-io") {
        b.define("MA_NO_DEVICE_IO", "1");
    }
    if cfg!(feature = "no-stdio") {
        b.define("MA_NO_STDIO", "1");
    }
    if cfg!(feature = "no-sse2") {
        b.define("MA_NO_SSE2", "1");
    }
    if cfg!(feature = "no-avx2") {
        b.define("MA_NO_AVX2", "1");
    }
    if cfg!(feature = "no-avx512") {
        b.define("MA_NO_AVX512", "1");
    }
    if cfg!(feature = "no-neon") {
        b.define("MA_NO_NEON", "1");
    }

    if cfg!(feature = "debug-output") {
        b.define("MA_DEBUG_OUTPUT", "1");
    }

    let mut log_level: Option<&'static str> = None;
    const LOG_LEVEL_VERBOSE: &'static str = "4";
    const LOG_LEVEL_INFO: &'static str = "3";
    const LOG_LEVEL_WARNING: &'static str = "2";
    const LOG_LEVEL_ERROR: &'static str = "1";

    if cfg!(feature = "log-level-error") {
        log_level = Some(LOG_LEVEL_ERROR);
    }
    if cfg!(feature = "log-level-warning") {
        log_level = Some(LOG_LEVEL_WARNING);
    }
    if cfg!(feature = "log-level-info") {
        log_level = Some(LOG_LEVEL_INFO);
    }
    if cfg!(feature = "log-level-verbose") {
        log_level = Some(LOG_LEVEL_VERBOSE);
    }

    if let Some(level) = log_level {
        b.define("MA_LOG_LEVEL", level);
    }
}

fn apply_flags(b: &mut cc::Build) {
    if cfg!(target_feature = "sse2") && !(cfg!(feature = "no-sse2")) {
        b.flag_if_supported("-msse2");
    }

    if cfg!(target_feature = "avx2") && !(cfg!(feature = "no-avx2")) {
        b.flag_if_supported("-mavx2");
    }

    if cfg!(target_feature = "avx512") && !(cfg!(feature = "no-avx512")) {
        b.flag_if_supported("-mavx512");
    }

    if cfg!(target_feature = "neon") && !(cfg!(feature = "no-neon")) {
        b.flag_if_supported("-mneon");
    }
}
