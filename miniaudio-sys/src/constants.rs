use std::os::raw::c_int;

/* Standard sample rates. */
pub const MA_SAMPLE_RATE_8000: c_int = 8000;
pub const MA_SAMPLE_RATE_11025: c_int = 11025;
pub const MA_SAMPLE_RATE_16000: c_int = 16000;
pub const MA_SAMPLE_RATE_22050: c_int = 22050;
pub const MA_SAMPLE_RATE_24000: c_int = 24000;
pub const MA_SAMPLE_RATE_32000: c_int = 32000;
pub const MA_SAMPLE_RATE_44100: c_int = 44100;
pub const MA_SAMPLE_RATE_48000: c_int = 48000;
pub const MA_SAMPLE_RATE_88200: c_int = 88200;
pub const MA_SAMPLE_RATE_96000: c_int = 96000;
pub const MA_SAMPLE_RATE_176400: c_int = 176400;
pub const MA_SAMPLE_RATE_192000: c_int = 192000;
pub const MA_SAMPLE_RATE_352800: c_int = 352800;
pub const MA_SAMPLE_RATE_384000: c_int = 384000;

/// For simplicity, miniaudio does not support PCM samples that are not byte aligned.
pub const MA_MIN_PCM_SAMPLE_SIZE_IN_BYTES: c_int = 1;
pub const MA_MAX_PCM_SAMPLE_SIZE_IN_BYTES: c_int = 8;
pub const MA_MIN_CHANNELS: c_int = 1;
pub const MA_MAX_CHANNELS: c_int = 32;
pub const MA_MIN_SAMPLE_RATE: c_int = MA_SAMPLE_RATE_8000;
pub const MA_MAX_SAMPLE_RATE: c_int = MA_SAMPLE_RATE_384000;
pub const MA_SRC_SINC_MIN_WINDOW_WIDTH: c_int = 2;
pub const MA_SRC_SINC_MAX_WINDOW_WIDTH: c_int = 32;
pub const MA_SRC_SINC_DEFAULT_WINDOW_WIDTH: c_int = 32;
pub const MA_SRC_SINC_LOOKUP_TABLE_RESOLUTION: c_int = 8;
pub const MA_SRC_INPUT_BUFFER_SIZE_IN_SAMPLES: c_int = 256;
