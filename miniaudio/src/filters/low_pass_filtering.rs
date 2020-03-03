use super::biquad_filtering::Biquad;
use crate::base::{Error, Format};
use crate::frames::{Frame, Frames, Sample};
use miniaudio_sys as sys;
use std::marker::PhantomData;

/// Configuration for a first order low-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct LPF1Config<S: Sample, F: Frame>(sys::ma_lpf1_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> LPF1Config<S, F> {
    pub fn new(sample_rate: u32, cutoff_frequency: f64) -> LPF1Config<S, F> {
        LPF1Config(
            unsafe {
                sys::ma_lpf1_config_init(
                    S::format() as _,
                    S::channels::<F>() as u32,
                    sample_rate,
                    cutoff_frequency,
                )
            },
            PhantomData,
            PhantomData,
        )
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
    pub fn q(&self) -> f64 {
        self.0.q
    }

    #[inline]
    pub fn set_q(&mut self, q: f64) {
        self.0.q = q;
    }
}

/// Configuration for a second order low-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct LPF2Config<S: Sample, F: Frame>(sys::ma_lpf1_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> LPF2Config<S, F> {
    pub fn new(sample_rate: u32, cutoff_frequency: f64, q: f64) -> LPF2Config<S, F> {
        LPF2Config(
            unsafe {
                sys::ma_lpf2_config_init(
                    S::format() as _,
                    S::channels::<F>() as u32,
                    sample_rate,
                    cutoff_frequency,
                    q,
                )
            },
            PhantomData,
            PhantomData,
        )
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
    pub fn q(&self) -> f64 {
        self.0.q
    }

    #[inline]
    pub fn set_q(&mut self, q: f64) {
        self.0.q = q;
    }
}

/// First order low-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct LPF1<S: Sample, F: Frame>(sys::ma_lpf1, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> LPF1<S, F> {
    pub fn new(config: &LPF1Config<S, F>) -> Result<LPF1<S, F>, Error> {
        let mut lpf1 = std::mem::MaybeUninit::<LPF1<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_lpf1_init(
                config as *const LPF1Config<S, F> as *const _,
                lpf1.as_mut_ptr() as *mut _,
            ))?;
            Ok(lpf1.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &LPF1Config<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf1_reinit(config as *const LPF1Config<S, F> as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.count(), input.count());
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_lpf1_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_lpf1_get_latency(&self.0 as *const _ as *mut _) }
    }
}

/// Second order low-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct LPF2<S: Sample, F: Frame>(sys::ma_lpf2, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> LPF2<S, F> {
    pub fn new(config: &LPF2Config<S, F>) -> Result<LPF2<S, F>, Error> {
        let mut lpf2 = std::mem::MaybeUninit::<LPF2<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_lpf2_init(
                config as *const LPF2Config<S, F> as *const _,
                lpf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(lpf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &LPF2Config<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_lpf2_reinit(config as *const LPF2Config<S, F> as *const _, &mut self.0)
        })
    }

    pub fn biquad(&self) -> &Biquad<S, F> {
        unsafe {
            (&self.0.bq as *const _ as *const Biquad<S, F>)
                .as_ref()
                .unwrap()
        }
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.count(), input.count());
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_lpf2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
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
    /// If order is set to 0, this will be treated as a passthrough (no filtering will be applied).
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
        order: u32,
    ) -> LPFConfig {
        LPFConfig(unsafe {
            sys::ma_lpf_config_init(format as _, channels, sample_rate, cutoff_frequency, order)
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
    pub fn order(&self) -> u32 {
        self.0.order
    }

    #[inline]
    pub fn set_order(&mut self, order: u32) {
        self.0.order = order;
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

    #[inline]
    pub fn process_pcm_frames<S: Sample + Copy + Sized, F: Copy + Sized>(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.count(), input.count());
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_lpf_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_lpf_get_latency(&self.0 as *const _ as *mut _) }
    }
}
