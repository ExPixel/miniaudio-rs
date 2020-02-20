use super::base::*;
use crate::constants::*;
use libc::{c_double, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub union BiquadCoefficient {
    pub float32: f32,
    pub int32: u32,
}
impl_void_debug!(BiquadCoefficient);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BiquadConfig {
    pub format: Format,
    pub channels: u32,
    pub b0: c_double,
    pub b1: c_double,
    pub b2: c_double,
    pub a0: c_double,
    pub a1: c_double,
    pub a2: c_double,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Biquad {
    pub format: Format,
    pub channels: u32,
    pub b0: BiquadCoefficient,
    pub b1: BiquadCoefficient,
    pub b2: BiquadCoefficient,
    pub a1: BiquadCoefficient,
    pub a2: BiquadCoefficient,
    pub r1: [BiquadCoefficient; MA_MAX_CHANNELS],
    pub r2: [BiquadCoefficient; MA_MAX_CHANNELS],
}
impl_void_debug!(Biquad); // FIXME real debug implementation

extern "C" {
    pub fn ma_biquad_config_init(
        format: Format,
        channels: u32,
        b0: c_double,
        b1: c_double,
        b2: c_double,
        a0: c_double,
        a1: c_double,
        a2: c_double,
    ) -> BiquadConfig;

    #[must_use]
    pub fn ma_biquad_init(config: *const BiquadConfig, bq: *mut Biquad) -> Result;

    #[must_use]
    pub fn ma_biquad_reinit(config: *const BiquadConfig, bq: *mut Biquad) -> Result;

    #[must_use]
    pub fn ma_biquad_process_pcm_frames(
        bq: *mut Biquad,
        frames_out: *mut c_void,
        frames_in: *const c_void,
        frame_count: u64,
    ) -> Result;

    pub fn ma_biquad_get_latency(bg: *mut Biquad) -> u32;
}
