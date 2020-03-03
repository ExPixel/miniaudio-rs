use super::biquad_filtering::Biquad;
use crate::base::{Error, Format};
use crate::frames::{Frame, Frames, Sample};
use miniaudio_sys as sys;
use std::marker::PhantomData;

/// Configuration for a second order high shelf filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HighShelf2Config<S: Sample, F: Frame>(
    sys::ma_hishelf2_config,
    PhantomData<S>,
    PhantomData<F>,
);

impl<S: Sample, F: Frame> HighShelf2Config<S, F> {
    #[inline]
    pub fn new(
        sample_rate: u32,
        gain_db: f64,
        shelf_slope: f64,
        frequency: f64,
    ) -> HighShelf2Config<S, F> {
        HighShelf2Config(
            unsafe {
                sys::ma_hishelf2_config_init(
                    S::format() as _,
                    S::channels::<F>() as _,
                    sample_rate,
                    gain_db,
                    shelf_slope,
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
    pub fn frequency(&self) -> f64 {
        self.0.frequency
    }

    #[inline]
    pub fn set_frequency(&mut self, frequency: f64) {
        self.0.frequency = frequency;
    }

    #[inline]
    pub fn gain_db(&self) -> f64 {
        self.0.gainDB
    }

    #[inline]
    pub fn set_gain_db(&mut self, gain_db: f64) {
        self.0.gainDB = gain_db;
    }

    #[inline]
    pub fn shelf_slope(&self) -> f64 {
        self.0.shelfSlope
    }

    #[inline]
    pub fn set_shelf_slope(&mut self, shelf_slope: f64) {
        self.0.shelfSlope = shelf_slope;
    }
}

/// Second order high pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HighShelf2<S: Sample, F: Frame>(sys::ma_hishelf2, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> HighShelf2<S, F> {
    #[inline]
    pub fn new(config: &HighShelf2Config<S, F>) -> Result<HighShelf2<S, F>, Error> {
        let mut hishelf2 = std::mem::MaybeUninit::<HighShelf2<S, F>>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hishelf2_init(
                config as *const HighShelf2Config<S, F> as *const _,
                hishelf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(hishelf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HighShelf2Config<S, F>) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hishelf2_reinit(
                config as *const HighShelf2Config<S, F> as *const _,
                &mut self.0,
            )
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
            sys::ma_hishelf2_process_pcm_frames(
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
        unsafe { sys::ma_hishelf2_get_latency(&self.0 as *const _ as *mut _) }
    }
}
