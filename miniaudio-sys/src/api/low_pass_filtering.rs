use super::base::*;
use super::biquad_filtering::Biquad;
use libc::{c_double, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LPFConfig {
    pub format: Format,
    pub channels: u32,
    pub sample_rate: u32,
    pub cutoff_frequency: c_double,
}

/// Low pass filter.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LPF {
    /// The low-pass filter is implemented as a biquad filter.
    pub bq: Biquad,
}

extern "C" {
    pub fn ma_lpf_config_init(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: c_double,
    ) -> LPFConfig;

    #[must_use]
    pub fn ma_lpf_init(config: *const LPFConfig, lpf: *mut LPF) -> Result;
    #[must_use]
    pub fn ma_lpf_reinit(config: *const LPFConfig, lpf: *mut LPF) -> Result;

    #[must_use]
    pub fn ma_lpf_process_pcm_frames(
        lpf: *mut LPF,
        frames_out: *mut c_void,
        frames_in: *const c_void,
        frame_count: u64,
    );

    pub fn ma_lpf_get_latency(lpf: *mut LPF) -> u32;
}
