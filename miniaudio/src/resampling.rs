use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use miniaudio_sys as sys;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResampleAlgorithmType {
    Linear = sys::ma_resample_algorithm_linear as _,
    Speex = sys::ma_resample_algorithm_speex as _,
}
impl_from_c!(ResampleAlgorithmType, sys::ma_resample_algorithm);

/// The choice of resampling algorithm depends on your situation and requirements.
/// The linear resampler is the most efficient and has the least amount of latency,
/// but at the expense of poorer quality. The Speex resampler is higher quality,
/// but slower with more latency. It also performs several heap allocations internally
/// for memory management.
#[derive(Clone, Copy, PartialEq)]
pub enum ResampleAlgorithm {
    Linear {
        lpf_order: u32,
        lpf_nyquist_factor: f64,
    },

    Speex {
        quality: u32,
    },
}

impl ResampleAlgorithm {
    pub fn algorithm_type(&self) -> ResampleAlgorithmType {
        match *self {
            ResampleAlgorithm::Linear { .. } => ResampleAlgorithmType::Linear,
            ResampleAlgorithm::Speex { .. } => ResampleAlgorithmType::Speex,
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct LinearResamplerConfig(sys::ma_linear_resampler_config);

impl LinearResamplerConfig {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> LinearResamplerConfig {
        LinearResamplerConfig(unsafe {
            sys::ma_linear_resampler_config_init(
                format as _,
                channels,
                sample_rate_in,
                sample_rate_out,
            )
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn sample_rate_in(&self) -> u32 {
        self.0.sampleRateIn
    }

    #[inline]
    pub fn set_sample_rate_in(&mut self, sample_rate: u32) {
        self.0.sampleRateIn = sample_rate;
    }

    #[inline]
    pub fn sample_rate_out(&self) -> u32 {
        self.0.sampleRateOut
    }

    #[inline]
    pub fn set_sample_rate_out(&mut self, sample_rate: u32) {
        self.0.sampleRateOut = sample_rate;
    }

    #[inline]
    pub fn lpf_nyquist_factor(&self) -> f64 {
        self.0.lpfNyquistFactor
    }

    #[inline]
    pub fn set_lpf_nyquist_factor(&mut self, factor: f64) {
        self.0.lpfNyquistFactor = factor;
    }

    #[inline]
    pub fn lpf_order(&self) -> u32 {
        self.0.lpfOrder
    }

    #[inline]
    pub fn set_lpf_order(&mut self, order: u32) {
        self.0.lpfOrder = order;
    }
}

#[repr(transparent)]
pub struct LinearResampler(sys::ma_linear_resampler);

impl LinearResampler {
    #[inline]
    pub fn new(config: &LinearResamplerConfig) -> Result<LinearResampler, Error> {
        let mut lr = std::mem::MaybeUninit::<LinearResampler>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_linear_resampler_init(
                config as *const LinearResamplerConfig as *const _,
                lr.as_mut_ptr() as *mut _,
            ))?;
            Ok(lr.assume_init())
        }
    }

    #[inline]
    pub fn config(&self) -> &LinearResamplerConfig {
        unsafe {
            &*(&self.0.config as *const sys::ma_linear_resampler_config
                as *const LinearResamplerConfig)
        }
    }

    // FIXME this API actually allows passing null for input or output and does this:
    //
    //      You can pass in NULL for the input buffer in which case it will be treated
    //      as an infinitely large buffer of zeros. The output buffer can also be NULL,
    //      in which case the processing will be treated as seek.
    //
    // I don't have a really good way to represent this right now, so I don't support it :P.
    //
    /// Converts the given input data.
    ///
    /// Returns the number of input frames that were consumed during processing and the number of
    /// output frames that were written to the output buffer respectively.
    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut FramesMut,
        input: &Frames,
    ) -> Result<(u64, u64), Error> {
        let mut output_frames = output.frame_count() as u64;
        let mut input_frames = input.frame_count() as u64;

        Error::from_c_result(unsafe {
            sys::ma_linear_resampler_process_pcm_frames(
                &mut self.0,
                input.as_ptr() as *const _,
                &mut input_frames,
                output.as_mut_ptr() as *mut _,
                &mut output_frames,
            )
        })?;

        return Ok((output_frames, input_frames));
    }

    /// Sets the input and output sample rate.
    #[inline]
    pub fn set_rate(&mut self, sample_rate_in: u32, sample_rate_out: u32) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_linear_resampler_set_rate(&mut self.0, sample_rate_in, sample_rate_out)
        })
    }

    /// Sets the input and output sample rate as a ratio.
    ///
    /// The ratio is in/out.
    #[inline]
    pub fn set_rate_ratio(&mut self, ratio_in_out: f32) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_linear_resampler_set_rate_ratio(&mut self.0, ratio_in_out)
        })
    }

    /// Calculates the number of whole input frames that would need to be read from the client in
    /// order to output the specified number of output frames.
    ///
    /// The returned value does not include cached input frames. It only returns the number of
    /// extra frames that would need to be read from the input buffer in order to output the
    /// specified number of output frames.
    #[inline]
    pub fn required_input_frame_count(&self, output_frame_count: u64) -> u64 {
        unsafe {
            sys::ma_linear_resampler_get_required_input_frame_count(
                &self.0 as *const _ as *mut _,
                output_frame_count,
            )
        }
    }

    /// Calculates the number of whole output frames that would be output after fully reading and
    /// consuming the specified number of input frames.
    #[inline]
    pub fn expected_output_frame_count(&self, input_frame_count: u64) -> u64 {
        unsafe {
            sys::ma_linear_resampler_get_expected_output_frame_count(
                &self.0 as *const _ as *mut _,
                input_frame_count,
            )
        }
    }
    #[inline]

    /// Retrieves the latency introduced by the resampler in input frames.
    pub fn input_latency(&mut self) -> u64 {
        unsafe { sys::ma_linear_resampler_get_input_latency(&self.0 as *const _ as *mut _) }
    }

    /// Retrieves the latency introduced by the resampler in output frames.
    #[inline]
    pub fn output_latency(&mut self) -> u64 {
        unsafe { sys::ma_linear_resampler_get_output_latency(&self.0 as *const _ as *mut _) }
    }

    #[inline]
    pub fn in_advance_int(&self) -> u32 {
        self.0.inAdvanceInt
    }

    #[inline]
    pub fn in_advance_frac(&self) -> u32 {
        self.0.inAdvanceFrac
    }

    #[inline]
    pub fn in_time_int(&self) -> u32 {
        self.0.inTimeInt
    }

    #[inline]
    pub fn in_time_frac(&self) -> u32 {
        self.0.inTimeFrac
    }
}

impl Drop for LinearResampler {
    fn drop(&mut self) {
        unsafe { sys::ma_linear_resampler_uninit(self as *mut LinearResampler as *mut _) };
    }
}

impl Clone for LinearResampler {
    fn clone(&self) -> Self {
        // This shouldn't fail assuming this was initialized properly to start with.
        Self::new(self.config()).expect("failed to clone linear resampler")
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ResamplerConfig(sys::ma_resampler_config);

impl ResamplerConfig {
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate_in: u32,
        sample_rate_out: u32,
        algorithm: ResampleAlgorithmType,
    ) -> ResamplerConfig {
        ResamplerConfig(unsafe {
            sys::ma_resampler_config_init(
                format as _,
                channels,
                sample_rate_in,
                sample_rate_out,
                algorithm as _,
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
    pub fn sample_rate_in(&self) -> u32 {
        self.0.sampleRateIn
    }

    #[inline]
    pub fn set_sample_rate_in(&mut self, sample_rate: u32) {
        self.0.sampleRateIn = sample_rate;
    }

    #[inline]
    pub fn sample_rate_out(&self) -> u32 {
        self.0.sampleRateOut
    }

    #[inline]
    pub fn set_sample_rate_out(&mut self, sample_rate: u32) {
        self.0.sampleRateOut = sample_rate;
    }

    pub fn set_algorithm(&mut self, algo: ResampleAlgorithm) {
        match algo {
            ResampleAlgorithm::Linear {
                lpf_order,
                lpf_nyquist_factor,
            } => {
                self.0.algorithm = sys::ma_resample_algorithm_linear;
                self.0.linear.lpfOrder = lpf_order;
                self.0.linear.lpfNyquistFactor = lpf_nyquist_factor;
            }

            ResampleAlgorithm::Speex { quality } => {
                self.0.algorithm = sys::ma_resample_algorithm_speex;
                self.0.speex.quality = quality as _;
            }
        }
    }

    pub fn algorithm(&self) -> ResampleAlgorithm {
        match self.0.algorithm {
            sys::ma_resample_algorithm_linear => ResampleAlgorithm::Linear {
                lpf_order: self.0.linear.lpfOrder,
                lpf_nyquist_factor: self.0.linear.lpfNyquistFactor,
            },

            sys::ma_resample_algorithm_speex => ResampleAlgorithm::Speex {
                quality: self.0.speex.quality as _,
            },

            _ => unreachable!(),
        }
    }
}

#[repr(transparent)]
pub struct Resampler(sys::ma_resampler);

impl Resampler {
    pub fn new(config: &ResamplerConfig) -> Result<Resampler, Error> {
        let mut resampler = std::mem::MaybeUninit::<Resampler>::uninit();
        unsafe {
            sys::ma_resampler_init(&config.0, resampler.as_mut_ptr() as *mut _);
            Ok(resampler.assume_init())
        }
    }

    #[inline]
    pub fn config(&self) -> &ResamplerConfig {
        unsafe { &*(&self.0.config as *const sys::ma_resampler_config as *const ResamplerConfig) }
    }

    // FIXME this API actually allows passing null for input or output and does this:
    //
    //      You can pass in NULL for the input buffer in which case it will be treated
    //      as an infinitely large buffer of zeros. The output buffer can also be NULL,
    //      in which case the processing will be treated as seek.
    //
    // I don't have a really good way to represent this right now, so I don't support it :P.
    //
    /// Converts the given input data.
    ///
    /// Returns the number of input frames that were consumed during processing and the number of
    /// output frames that were written to the output buffer respectively.
    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut FramesMut,
        input: &Frames,
    ) -> Result<(u64, u64), Error> {
        if output.format() != input.format() {
            ma_debug_panic!(
                "output and input format did not match (output: {:?}, input: {:?}",
                output.format(),
                input.format()
            );
            return Err(Error::InvalidArgs);
        }

        let mut output_frames = output.frame_count() as u64;
        let mut input_frames = input.frame_count() as u64;

        Error::from_c_result(unsafe {
            sys::ma_resampler_process_pcm_frames(
                &mut self.0,
                input.as_ptr() as *const _,
                &mut input_frames,
                output.as_mut_ptr() as *mut _,
                &mut output_frames,
            )
        })?;

        return Ok((input_frames, output_frames));
    }

    /// Sets the input and output sample rate.
    #[inline]
    pub fn set_rate(&mut self, sample_rate_in: u32, sample_rate_out: u32) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_resampler_set_rate(&mut self.0, sample_rate_in, sample_rate_out)
        })
    }

    /// Sets the input and output sample rate as a ratio.
    ///
    /// The ratio is in/out.
    #[inline]
    pub fn set_rate_ratio(&mut self, ratio_in_out: f32) -> Result<(), Error> {
        Error::from_c_result(unsafe { sys::ma_resampler_set_rate_ratio(&mut self.0, ratio_in_out) })
    }

    /// Calculates the number of whole input frames that would need to be read from the client in
    /// order to output the specified number of output frames.
    ///
    /// The returned value does not include cached input frames. It only returns the number of
    /// extra frames that would need to be read from the input buffer in order to output the
    /// specified number of output frames.
    #[inline]
    pub fn required_input_frame_count(&self, output_frame_count: u64) -> u64 {
        unsafe {
            sys::ma_resampler_get_required_input_frame_count(
                &self.0 as *const _ as *mut _,
                output_frame_count,
            )
        }
    }

    /// Calculates the number of whole output frames that would be output after fully reading and
    /// consuming the specified number of input frames.
    #[inline]
    pub fn expected_output_frame_count(&self, input_frame_count: u64) -> u64 {
        unsafe {
            sys::ma_linear_resampler_get_expected_output_frame_count(
                &self.0 as *const _ as *mut _,
                input_frame_count,
            )
        }
    }
    #[inline]

    /// Retrieves the latency introduced by the resampler in input frames.
    pub fn input_latency(&mut self) -> u64 {
        unsafe { sys::ma_linear_resampler_get_input_latency(&self.0 as *const _ as *mut _) }
    }

    /// Retrieves the latency introduced by the resampler in output frames.
    #[inline]
    pub fn output_latency(&mut self) -> u64 {
        unsafe { sys::ma_linear_resampler_get_output_latency(&self.0 as *const _ as *mut _) }
    }
}

impl Clone for Resampler {
    fn clone(&self) -> Self {
        // This should not fail if the resampler was properly initialized.
        Self::new(self.config()).expect("failed to clone resampler")
    }
}

impl Drop for Resampler {
    fn drop(&mut self) {
        unsafe { sys::ma_resampler_uninit(&mut self.0) };
    }
}
