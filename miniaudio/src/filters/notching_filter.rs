use super::biquad_filtering::Biquad;
use crate::base::{Error, Format};
use crate::frames::{Frame, Frames, Sample};
use miniaudio_sys as sys;
use std::marker::PhantomData;

/// Configuration for a second order notching filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct Notch2Config<S: Sample, F: Frame>(sys::ma_notch2_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> Notch2Config<S, F> {
    #[inline]
    pub fn new(sample_rate: u32, q: f64, frequency: f64) -> Notch2Config<S, F> {
        Notch2Config(
            unsafe {
                sys::ma_notch2_config_init(
                    S::format() as _,
                    S::channels::<F>() as _,
                    sample_rate,
                    q,
                    frequency,
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
    pub fn frequency(&self) -> f64 {
        self.0.frequency
    }

    #[inline]
    pub fn set_frequency(&mut self, frequency: f64) {
        self.0.frequency = frequency;
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

/// Second order notching filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct Notch2<S: Sample, F: Frame>(sys::ma_notch2, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> Notch2<S, F> {
    #[inline]
    pub fn new(config: &Notch2Config<S, F>) -> Result<Notch2<S, F>, Error> {
        let mut notch2 = std::mem::MaybeUninit::<Notch2<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_notch2_init(
                config as *const Notch2Config<S, F> as *const _,
                notch2.as_mut_ptr() as *mut _,
            ))?;
            Ok(notch2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &Notch2Config<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_notch2_reinit(config as *const Notch2Config<S, F> as *const _, &mut self.0)
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
            sys::ma_notch2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        })
    }

    #[inline]
    pub fn bq(&self) -> &Biquad<S, F> {
        unsafe { std::mem::transmute(&self.0.bq) }
    }

    #[inline]
    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_notch2_get_latency(&self.0 as *const _ as *mut _) }
    }
}
