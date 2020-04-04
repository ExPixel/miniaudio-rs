use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use crate::resampling::ResampleAlgorithm;
use miniaudio_sys as sys;

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
