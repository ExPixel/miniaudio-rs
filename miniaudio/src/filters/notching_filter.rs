use super::biquad_filtering::Biquad;
use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use miniaudio_sys as sys;

/// Configuration for a second order notching filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct Notch2Config(sys::ma_notch2_config);

impl Notch2Config {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        q: f64,
        frequency: f64,
    ) -> Notch2Config {
        Notch2Config(unsafe {
            sys::ma_notch2_config_init(format as _, channels, sample_rate, q, frequency)
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
pub struct Notch2(sys::ma_notch2);

impl Notch2 {
    #[inline]
    pub fn new(config: &Notch2Config) -> Result<Notch2, Error> {
        let mut notch2 = std::mem::MaybeUninit::<Notch2>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_notch2_init(
                config as *const Notch2Config as *const _,
                notch2.as_mut_ptr() as *mut _,
            ))?;
            Ok(notch2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &Notch2Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_notch2_reinit(config as *const Notch2Config as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut FramesMut,
        input: &Frames,
    ) -> Result<(), Error> {
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
            sys::ma_notch2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }

    #[inline]
    pub fn bq(&self) -> &Biquad {
        unsafe { std::mem::transmute(&self.0.bq) }
    }

    #[inline]
    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_notch2_get_latency(&self.0 as *const _ as *mut _) }
    }
}
