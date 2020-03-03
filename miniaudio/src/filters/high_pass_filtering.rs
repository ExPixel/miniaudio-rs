use super::biquad_filtering::Biquad;
use crate::base::{Error, Format};
use crate::frames::{Frame, Frames, Sample};
use miniaudio_sys as sys;
use std::marker::PhantomData;

/// Configuration for a first order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF1Config<S: Sample, F: Frame>(sys::ma_hpf1_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HPF1Config<S, F> {
    pub fn new(sample_rate: u32, cutoff_frequency: f64) -> HPF1Config<S, F> {
        HPF1Config(
            unsafe {
                sys::ma_hpf1_config_init(
                    S::format() as _,
                    S::channels::<F>() as _,
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

/// Configuration for a second order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF2Config<S: Sample, F: Frame>(sys::ma_hpf1_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HPF2Config<S, F> {
    pub fn new(sample_rate: u32, cutoff_frequency: f64, q: f64) -> HPF2Config<S, F> {
        HPF2Config(
            unsafe {
                sys::ma_hpf2_config_init(
                    S::format() as _,
                    S::channels::<F>() as _,
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

/// First order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF1<S: Sample, F: Frame>(sys::ma_hpf1, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HPF1<S, F> {
    pub fn new(config: &HPF1Config<S, F>) -> Result<HPF1<S, F>, Error> {
        let mut hpf1 = std::mem::MaybeUninit::<HPF1<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hpf1_init(
                config as *const HPF1Config<S, F> as *const _,
                hpf1.as_mut_ptr() as *mut _,
            ))?;
            Ok(hpf1.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HPF1Config<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hpf1_reinit(config as *const HPF1Config<S, F> as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_hpf1_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_hpf1_get_latency(&self.0 as *const _ as *mut _) }
    }
}

/// Second order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF2<S: Sample, F: Frame>(sys::ma_hpf2, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HPF2<S, F> {
    pub fn new(config: &HPF2Config<S, F>) -> Result<HPF2<S, F>, Error> {
        let mut hpf2 = std::mem::MaybeUninit::<HPF2<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hpf2_init(
                config as *const HPF2Config<S, F> as *const _,
                hpf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(hpf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HPF2Config<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hpf2_reinit(config as *const HPF2Config<S, F> as *const _, &mut self.0)
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
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_hpf2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_hpf2_get_latency(&self.0 as *const _ as *mut _) }
    }
}

/// Configuration for a high-pass filter with configurable order (up to 8)
#[repr(transparent)]
#[derive(Clone)]
pub struct HPFConfig<S: Sample, F: Frame>(sys::ma_hpf_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HPFConfig<S, F> {
    /// If order is set to 0, this will be treated as a passthrough (no filtering will be applied).
    pub fn new(sample_rate: u32, cutoff_frequency: f64, order: u32) -> HPFConfig<S, F> {
        HPFConfig(
            unsafe {
                sys::ma_hpf_config_init(
                    S::format() as _,
                    S::channels::<F>() as _,
                    sample_rate,
                    cutoff_frequency,
                    order,
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
    pub fn order(&self) -> u32 {
        self.0.order
    }

    #[inline]
    pub fn set_order(&mut self, order: u32) {
        self.0.order = order;
    }
}

/// High-pass filter with configurable order (up to 8)
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF<S: Sample, F: Frame>(sys::ma_hpf, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HPF<S, F> {
    pub fn new(config: &HPFConfig<S, F>) -> Result<HPF<S, F>, Error> {
        let mut hpf = std::mem::MaybeUninit::<HPF<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hpf_init(
                config as *const HPFConfig<S, F> as *const _,
                hpf.as_mut_ptr() as *mut _,
            ))?;
            Ok(hpf.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HPFConfig<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hpf_reinit(config as *const HPFConfig<S, F> as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        if output.count() != input.count() {
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_hpf_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_hpf_get_latency(&self.0 as *const _ as *mut _) }
    }
}
