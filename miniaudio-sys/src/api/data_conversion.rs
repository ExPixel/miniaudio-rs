use super::base::*;
use super::low_pass_filtering::LPF;
use crate::constants::*;
use libc::{c_double, c_float, c_void};

pub const MA_MAX_RESAMPLER_LPF_FILTERS: usize = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LinearResamplerConfig {
    pub format: Format,
    pub channels: u32,
    pub sample_rate_in: u32,
    pub sample_rate_out: u32,
    /// How many low-pass filters to chain together. A single low-pass filter is second order.
    /// Setting this to 0 will disable low-pass filtering.
    pub lpf_count: u32,
    /// 0..1. Defaults to 1. 1 = Half the sampling frequency (Nyquist Frequency), 0.5 = Quarter the sampling frequency (half Nyquest Frequency), etc.
    pub lpf_nyquist_factor: c_double,
}

extern "C" {
    pub fn ma_linear_resampler_config_init(
        format: Format,
        channels: u32,
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> LinearResamplerConfig;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LinearResampler {
    pub config: LinearResamplerConfig,
    pub in_advance_int: u32,
    pub in_advance_frac: u32,
    pub in_time_int: u32,
    pub in_time_frac: u32,
    /// The previous input frame.
    pub x0: LinearResamplerFrame,
    /// The next input frame.
    pub x1: LinearResamplerFrame,
    pub lpf: [LPF; MA_MAX_RESAMPLER_LPF_FILTERS],
}
impl_void_debug!(LinearResampler); // FIXME real debug implementation.

#[repr(C)]
#[derive(Clone, Copy)]
pub union LinearResamplerFrame {
    pub float32: [c_float; MA_MAX_CHANNELS],
    pub int16: [u16; MA_MAX_CHANNELS],
}
impl_void_debug!(LinearResamplerFrame); // FIXME

extern "C" {
    #[must_use]
    pub fn ma_linear_resampler_init(
        config: *const LinearResamplerConfig,
        resampler: *mut LinearResampler,
    ) -> Result;

    pub fn ma_linear_resampler_uninit(resampler: *mut LinearResampler);

    #[must_use]
    pub fn ma_linear_resampler_process_pcm_frames(
        resampler: *mut LinearResampler,
        frames_in: *const c_void,
        frame_count_in: *mut u64,
        frames_out: *mut c_void,
        frame_count_out: *mut u64,
    ) -> Result;

    #[must_use]
    pub fn ma_linear_resampler_set_rate(
        resample: *mut LinearResampler,
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> Result;

    #[must_use]
    pub fn ma_linear_resampler_set_rate_ratio(
        resampler: *mut LinearResampler,
        ration_in_out: c_float,
    ) -> Result;

    pub fn ma_linear_resampler_get_required_input_frame_count(
        resampler: *mut LinearResampler,
        output_frame_count: u64,
    ) -> u64;

    pub fn ma_linear_resampler_get_required_output_frame_count(
        resampler: *mut LinearResampler,
        input_frame_count: u64,
    ) -> u64;

    pub fn ma_linear_resampler_get_input_latency(resampler: *mut LinearResampler) -> u64;

    pub fn ma_linear_resampler_get_output_latency(resampler: *mut LinearResampler) -> u64;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum ResampleAlgorithm {
    /// Fastest, lowest quality. Optional low-pass filtering. Default.
    Linear = 0,
    Speex = 1,
}
impl_default!(ResampleAlgorithm, ResampleAlgorithm::Linear);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ResamplerConfig {
    /// Must be either F32 or S16.
    pub format: Format,
    pub channels: u32,
    pub sample_rate_in: u32,
    pub sample_rate_out: 32,
    pub algorithm: ResampleAlgorithm,
}
