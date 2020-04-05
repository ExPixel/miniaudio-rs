use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use crate::resampling::ResampleAlgorithm;
use miniaudio_sys as sys;

#[repr(transparent)]
pub struct DataConverterConfig(sys::ma_data_converter_config);

impl DataConverterConfig {
    pub fn new(
        format_in: Format,
        format_out: Format,
        channels_in: u32,
        channels_out: u32,
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> DataConverterConfig {
        DataConverterConfig(unsafe {
            sys::ma_data_converter_config_init(
                format_in as _,
                format_out as _,
                channels_in,
                channels_out,
                sample_rate_in,
                sample_rate_out,
            )
        })
    }

    #[inline]
    pub fn format_in(&self) -> Format {
        Format::from_c(self.0.formatIn)
    }

    #[inline]
    pub fn set_format_in(&mut self, format: Format) {
        self.0.formatIn = format as _;
    }
    #[inline]
    pub fn format_out(&self) -> Format {
        Format::from_c(self.0.formatOut)
    }

    #[inline]
    pub fn set_format_out(&mut self, format: Format) {
        self.0.formatOut = format as _;
    }

    #[inline]
    pub fn channels_in(&self) -> u32 {
        self.0.channelsIn
    }

    #[inline]
    pub fn set_channels_in(&mut self, channels: u32) {
        self.0.channelsIn = channels;
    }

    #[inline]
    pub fn channels_out(&self) -> u32 {
        self.0.channelsOut
    }

    #[inline]
    pub fn set_channels_out(&mut self, channels: u32) {
        self.0.channelsOut = channels;
    }

    pub fn sample_rate_in(&self) -> u32 {
        self.0.sampleRateIn
    }

    pub fn set_sample_rate_in(&mut self, rate: u32) {
        self.0.sampleRateIn = rate;
    }

    pub fn sample_rate_out(&self) -> u32 {
        self.0.sampleRateOut
    }

    pub fn set_resampling(&mut self, algo: ResampleAlgorithm) {
        match algo {
            ResampleAlgorithm::Linear {
                lpf_order,
                lpf_nyquist_factor,
            } => {
                self.0.resampling.algorithm = sys::ma_resample_algorithm_linear;
                self.0.resampling.linear.lpfOrder = lpf_order;
                self.0.resampling.linear.lpfNyquistFactor = lpf_nyquist_factor;
            }

            ResampleAlgorithm::Speex { quality } => {
                self.0.resampling.algorithm = sys::ma_resample_algorithm_speex;
                self.0.resampling.speex.quality = quality as _;
            }
        }
    }

    pub fn resampling(&self) -> ResampleAlgorithm {
        match self.0.resampling.algorithm {
            sys::ma_resample_algorithm_linear => ResampleAlgorithm::Linear {
                lpf_order: self.0.resampling.linear.lpfOrder,
                lpf_nyquist_factor: self.0.resampling.linear.lpfNyquistFactor,
            },

            sys::ma_resample_algorithm_speex => ResampleAlgorithm::Speex {
                quality: self.0.resampling.speex.quality as _,
            },

            _ => unreachable!(),
        }
    }
}

impl Default for DataConverterConfig {
    fn default() -> Self {
        DataConverterConfig(unsafe { sys::ma_data_converter_config_init_default() })
    }
}

#[repr(transparent)]
pub struct DataConverter(sys::ma_data_converter);

impl DataConverter {
    pub fn new(config: &DataConverterConfig) -> Result<DataConverter, Error> {
        let mut converter = std::mem::MaybeUninit::<DataConverter>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_data_converter_init(
                &config.0 as *const _,
                converter.as_mut_ptr().cast(),
            ))?;
            Ok(converter.assume_init())
        }
    }

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

        if output.byte_count() != input.byte_count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.frame_count(), input.frame_count());
            return Err(Error::InvalidArgs);
        }

        let mut input_frame_count = input.frame_count() as u64;
        let mut output_frame_count = output.frame_count() as u64;

        Error::from_c_result(unsafe {
            sys::ma_data_converter_process_pcm_frames(
                &mut self.0,
                input.as_ptr() as *const _,
                &mut input_frame_count,
                output.as_mut_ptr() as *mut _,
                &mut output_frame_count,
            )
        })?;

        return Ok((output_frame_count, input_frame_count));
    }

    pub fn set_rate(&mut self, sample_rate_in: u32, sample_rate_out: u32) -> Result<(), Error> {
        return Error::from_c_result(unsafe {
            sys::ma_data_converter_set_rate(&mut self.0, sample_rate_in, sample_rate_out)
        });
    }

    pub fn set_rate_ratio(&mut self, ratio_in_out: f32) -> Result<(), Error> {
        return Error::from_c_result(unsafe {
            sys::ma_data_converter_set_rate_ratio(&mut self.0, ratio_in_out)
        });
    }

    pub fn required_input_frame_count(&self, output_frame_count: u64) -> u64 {
        unsafe {
            sys::ma_data_converter_get_required_input_frame_count(
                &self as *const _ as *mut _,
                output_frame_count,
            )
        }
    }

    pub fn expected_output_frame_count(&self, input_frame_count: u64) -> u64 {
        unsafe {
            sys::ma_data_converter_get_expected_output_frame_count(
                &self as *const _ as *mut _,
                input_frame_count,
            )
        }
    }

    pub fn input_latency(&self) -> u64 {
        unsafe { sys::ma_data_converter_get_input_latency(&self as *const _ as *mut _) }
    }

    pub fn output_latency(&self) -> u64 {
        unsafe { sys::ma_data_converter_get_output_latency(&self as *const _ as *mut _) }
    }
}

impl Drop for DataConverter {
    fn drop(&mut self) {
        unsafe {
            sys::ma_data_converter_uninit(&mut self.0);
        }
    }
}
