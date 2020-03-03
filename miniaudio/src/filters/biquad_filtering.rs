use crate::base::*;
use crate::frames::{Frame, Frames, Sample};
use miniaudio_sys as sys;
use std::marker::PhantomData;

#[repr(transparent)]
#[derive(Clone)]
pub struct BiquadConfig<S: Sample, F: Frame>(sys::ma_biquad_config, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> BiquadConfig<S, F> {
    #[inline]
    pub fn new(numerators: [f64; 3], denominators: [f64; 3]) -> BiquadConfig<S, F> {
        unsafe {
            BiquadConfig(
                sys::ma_biquad_config_init(
                    S::format() as _,
                    S::channels::<F>() as _,
                    numerators[0],
                    numerators[1],
                    numerators[2],
                    denominators[0],
                    denominators[1],
                    denominators[2],
                ),
                PhantomData,
                PhantomData,
            )
        }
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    /// Get a Biquad Coefficient numerator.
    #[inline]
    pub fn b(&self, index: usize) -> f64 {
        match index {
            0 => self.0.b0,
            1 => self.0.b1,
            2 => self.0.b2,
            _ => panic!("Numerator out of range."),
        }
    }

    /// Set a Biquad Coefficient numerator.
    #[inline]
    pub fn set_b(&mut self, index: usize, value: f64) {
        match index {
            0 => self.0.b0 = value,
            1 => self.0.b1 = value,
            2 => self.0.b2 = value,
            _ => panic!("Numerator out of range."),
        }
    }

    /// Get a Biquad Coefficient denominator.
    #[inline]
    pub fn a(&self, index: usize) -> f64 {
        match index {
            0 => self.0.a0,
            1 => self.0.a1,
            2 => self.0.a2,
            _ => panic!("Denominator out of range."),
        }
    }

    /// Set a Biquad Coefficient denominator.
    #[inline]
    pub fn set_a(&mut self, index: usize, value: f64) {
        match index {
            0 => self.0.a0 = value,
            1 => self.0.a1 = value,
            2 => self.0.a2 = value,
            _ => panic!("Denominator out of range."),
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Biquad<S: Sample, F: Frame>(sys::ma_biquad, PhantomData<S>, PhantomData<F>);

impl<S: Sample, F: Frame> Biquad<S, F> {
    #[inline]
    pub fn new(config: &BiquadConfig<S, F>) -> Result<Biquad<S, F>, Error> {
        let mut biquad = std::mem::MaybeUninit::uninit();
        unsafe {
            let result = sys::ma_biquad_init(&config.0, biquad.as_mut_ptr());
            map_result!(
                result,
                Biquad(biquad.assume_init(), PhantomData, PhantomData)
            )
        }
    }

    #[inline]
    pub fn reinit(&mut self, config: &BiquadConfig<S, F>) -> Result<(), Error> {
        let result = unsafe { sys::ma_biquad_reinit(&config.0, &mut self.0) };
        Error::from_c_result(result)
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut Frames<S, F>,
        input: &Frames<S, F>,
    ) -> Result<(), Error> {
        // NOTE FrameType and SampleType being equal for both input and output frames means that we
        // basically assert that the number of channels and the format type for both streams are
        // equal for free. Still have to check the number of frames in each one though.
        if output.count() != input.count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.count(), input.count());
            return Err(Error::InvalidArgs);
        }

        let result = unsafe {
            sys::ma_biquad_process_pcm_frames(
                &mut self.0,
                output.frames_ptr_mut() as *mut _,
                input.frames_ptr() as *const _,
                output.count() as u64,
            )
        };
        Error::from_c_result(result)
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    // FIXME the pointer is not marked as const in the C source even though it could be. I should
    // probably try to get that fixed.
    #[inline]
    pub fn latency(&mut self) -> u32 {
        unsafe { sys::ma_biquad_get_latency(&mut self.0) }
    }
}
