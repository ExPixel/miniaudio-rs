use super::biquad_filtering::Biquad;
use super::Filter;
use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use miniaudio_sys as sys;

/// Configuration for a second order low shelf filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct LowShelf2Config(sys::ma_loshelf2_config);

impl LowShelf2Config {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        gain_db: f64,
        shelf_slope: f64,
        frequency: f64,
    ) -> LowShelf2Config {
        LowShelf2Config(unsafe {
            sys::ma_loshelf2_config_init(
                format as _,
                channels,
                sample_rate,
                gain_db,
                shelf_slope,
                frequency,
            )
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

/// Second order low shelf filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct LowShelf2(sys::ma_loshelf2);

impl LowShelf2 {
    #[inline]
    pub fn new(config: &LowShelf2Config) -> Result<LowShelf2, Error> {
        let mut loshelf2 = std::mem::MaybeUninit::<LowShelf2>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_loshelf2_init(
                config as *const LowShelf2Config as *const _,
                loshelf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(loshelf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &LowShelf2Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_loshelf2_reinit(config as *const LowShelf2Config as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn bq(&self) -> &Biquad {
        unsafe { &*(&self.0.bq as *const sys::ma_biquad as *const Biquad) }
    }

    #[inline]
    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_loshelf2_get_latency(&self.0 as *const _ as *mut _) }
    }
}

impl Filter for LowShelf2 {
    #[inline]
    fn process_pcm_frames(&mut self, output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
        if output.format() != input.format() {
            ma_debug_panic!(
                "output and input format did not match (output: {:?}, input: {:?}",
                output.format(),
                input.format()
            );
            return Err(Error::InvalidArgs);
        }

        if output.frame_count() != input.frame_count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.frame_count(), input.frame_count());
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_loshelf2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }
}
