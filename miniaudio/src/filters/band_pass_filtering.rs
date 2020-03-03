use super::biquad_filtering::Biquad;
use crate::base::{Error, Format, MAX_FILTER_ORDER};
use crate::frames::{Frames, Sample};
use miniaudio_sys as sys;

/// Second order band-pass filter config.
#[repr(transparent)]
#[derive(Clone)]
pub struct BPF2Config(sys::ma_bpf2_config);

impl BPF2Config {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
        q: f64,
    ) -> BPF2Config {
        unsafe {
            BPF2Config(sys::ma_bpf2_config_init(
                format as _,
                channels,
                sample_rate,
                cutoff_frequency,
                q,
            ))
        }
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
    pub fn set_cutoff_frequency(&mut self, frequency: f64) {
        self.0.cutoffFrequency = frequency;
    }

    #[inline]
    pub fn q(&self) -> f64 {
        self.0.q
    }

    #[inline]
    pub fn set_q(&mut self, q: f64) {
        self.0.q = q;
    }
}

/// Second order band-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct BPF2(sys::ma_bpf2);

impl BPF2 {
    #[inline]
    pub fn new(config: &BPF2Config) -> Result<BPF2, Error> {
        let mut bpf2 = std::mem::MaybeUninit::<BPF2>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_bpf2_init(
                config as *const BPF2Config as *const _,
                bpf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(bpf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &BPF2Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_bpf2_reinit(config as *const BPF2Config as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn process_pcm_frames<S: Sample + Copy + Sized, F: Copy + Sized>(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_bpf2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    #[inline]
    pub fn bq(&self) -> &Biquad {
        unsafe { std::mem::transmute(&self.0.bq) }
    }

    #[inline]
    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_bpf2_get_latency(&self.0 as *const _ as *mut _) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct BPFConfig(sys::ma_bpf_config);

impl BPFConfig {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
        order: u32,
    ) -> BPFConfig {
        unsafe {
            BPFConfig(sys::ma_bpf_config_init(
                format as _,
                channels,
                sample_rate,
                cutoff_frequency,
                order,
            ))
        }
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
    pub fn set_cutoff_frequency(&mut self, frequency: f64) {
        self.0.cutoffFrequency = frequency;
    }

    #[inline]
    pub fn order(&self) -> u32 {
        self.0.order
    }

    /// If set to 0, will be treated as a passthrough (no filtering will be applied).
    #[inline]
    pub fn set_order(&mut self, order: u32) {
        self.0.order = order;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct BPF(sys::ma_bpf);

impl BPF {
    #[inline]
    pub fn new(config: &BPFConfig) -> Result<BPF, Error> {
        let mut bpf = std::mem::MaybeUninit::<BPF>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_bpf_init(
                config as *const BPFConfig as *const _,
                bpf.as_mut_ptr() as *mut _,
            ))?;
            Ok(bpf.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &BPFConfig) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_bpf_reinit(config as *const BPFConfig as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn process_pcm_frames<S: Sample + Copy + Sized, F: Copy + Sized>(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_bpf_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn bpf2_count(&self) -> u32 {
        self.0.bpf2Count
    }

    #[inline]
    pub fn bpf2(&self) -> &[BPF2; MAX_FILTER_ORDER / 2] {
        unsafe { std::mem::transmute(&self.0.bpf2) }
    }

    #[inline]
    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_bpf_get_latency(&self.0 as *const _ as *mut _) }
    }
}
