use super::biquad_filtering::Biquad;
use crate::base::{Error, Format};
use miniaudio_sys as sys;
use std::ptr::NonNull;

#[repr(transparent)]
#[derive(Clone)]
pub struct LPF1Config(sys::ma_lpf1_config);

impl LPF1Config {
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
    ) -> LPF1Config {
        LPF1Config(unsafe {
            sys::ma_lpf1_config_init(format as _, channels, sample_rate, cutoff_frequency)
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.0.sampleRate = sample_rate;
    }

    #[inline]
    pub fn cutoff_frequency(&self) -> f64 {
        self.0.cutoffFrequency
    }

    #[inline]
    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f64) {
        self.0.cutoffFrequency = cutoff_frequency;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct LPF2Config(sys::ma_lpf1_config);

impl LPF2Config {
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
    ) -> LPF2Config {
        LPF2Config(unsafe {
            sys::ma_lpf2_config_init(format as _, channels, sample_rate, cutoff_frequency)
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.0.sampleRate = sample_rate;
    }

    #[inline]
    pub fn cutoff_frequency(&self) -> f64 {
        self.0.cutoffFrequency
    }

    #[inline]
    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f64) {
        self.0.cutoffFrequency = cutoff_frequency;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct LPF1(sys::ma_lpf1);

impl LPF1 {
    pub fn new(config: &LPF1Config) -> Result<LPF1, Error> {
        let mut lpf1 = std::mem::MaybeUninit::<LPF1>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_lpf1_init(
                config as *const LPF1Config as *const _,
                lpf1.as_mut_ptr() as *mut _,
            ))?;
            Ok(lpf1.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &LPF1Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf1_reinit(config as *const LPF1Config as *const _, &mut self.0)
        })
    }

    pub fn process_pcm_frames(
        &mut self,
        output: NonNull<()>,
        input: NonNull<()>,
        frame_count: u64,
    ) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf1_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_ptr() as *mut _,
                input.as_ptr() as *const _,
                frame_count,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_lpf1_get_latency(&self.0 as *const _ as *mut _) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct LPF2(sys::ma_lpf2);

impl LPF2 {
    pub fn new(config: &LPF2Config) -> Result<LPF2, Error> {
        let mut lpf2 = std::mem::MaybeUninit::<LPF2>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_lpf2_init(
                config as *const LPF2Config as *const _,
                lpf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(lpf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &LPF2Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf2_reinit(config as *const LPF2Config as *const _, &mut self.0)
        })
    }

    pub fn biquad(&self) -> &Biquad {
        unsafe { (&self.0.bq as *const _ as *const Biquad).as_ref().unwrap() }
    }

    pub fn process_pcm_frames(
        &mut self,
        output: NonNull<()>,
        input: NonNull<()>,
        frame_count: u64,
    ) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_ptr() as *mut _,
                input.as_ptr() as *const _,
                frame_count,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_lpf2_get_latency(&self.0 as *const _ as *mut _) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct LPFConfig(sys::ma_lpf_config);

impl LPFConfig {
    /// If poles is set to 0, this will be treated as a passthrough (no filtering will be applied).
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
        poles: u32,
    ) -> LPFConfig {
        LPFConfig(unsafe {
            sys::ma_lpf_config_init(format as _, channels, sample_rate, cutoff_frequency, poles)
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.0.sampleRate = sample_rate;
    }

    #[inline]
    pub fn cutoff_frequency(&self) -> f64 {
        self.0.cutoffFrequency
    }

    #[inline]
    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f64) {
        self.0.cutoffFrequency = cutoff_frequency;
    }

    #[inline]
    pub fn poles(&self) -> u32 {
        self.0.poles
    }

    #[inline]
    pub fn set_poles(&mut self, poles: u32) {
        self.0.poles = poles;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct LPF(sys::ma_lpf);

impl LPF {
    pub fn new(config: &LPFConfig) -> Result<LPF, Error> {
        let mut lpf = std::mem::MaybeUninit::<LPF>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_lpf_init(
                config as *const LPFConfig as *const _,
                lpf.as_mut_ptr() as *mut _,
            ))?;
            Ok(lpf.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &LPFConfig) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf_reinit(config as *const LPFConfig as *const _, &mut self.0)
        })
    }

    pub fn process_pcm_frames(
        &mut self,
        output: NonNull<()>,
        input: NonNull<()>,
        frame_count: u64,
    ) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_ptr() as *mut _,
                input.as_ptr() as *const _,
                frame_count,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_lpf_get_latency(&self.0 as *const _ as *mut _) }
    }
}
