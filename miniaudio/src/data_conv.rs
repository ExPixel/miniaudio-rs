use crate::base::{Error, Format, MAX_FILTER_ORDER};
use crate::frames::{Frame, Frames, Sample};
use crate::resampling::ResampleAlgorithm;
use miniaudio_sys as sys;
use std::marker::PhantomData;

pub struct DataConverterConfig<Sin: Sample, Fin: Frame, Sout: Sample, Fout: Frame>(
    sys::ma_data_converter_config,
    PhantomData<(Sin, Fin, Sout, Fout)>,
);

impl<Sin: Sample, Fin: Frame, Sout: Sample, Fout: Frame> DataConverterConfig<Sin, Fin, Sout, Fout> {
    pub fn new(
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> DataConverterConfig<Sin, Fin, Sout, Fout> {
        DataConverterConfig(
            unsafe {
                sys::ma_data_converter_config_init(
                    Sin::format() as _,
                    Sout::format() as _,
                    Sin::channels::<Fin>() as _,
                    Sout::channels::<Fout>() as _,
                    sample_rate_in,
                    sample_rate_out,
                )
            },
            PhantomData,
        )
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

impl<Sin: Sample, Fin: Frame, Sout: Sample, Fout: Frame> Default
    for DataConverterConfig<Sin, Fin, Sout, Fout>
{
    fn default() -> Self {
        let mut raw_config = unsafe { sys::ma_data_converter_config_init_default() };
        raw_config.formatIn = Sin::format() as _;
        raw_config.channelsIn = Sin::channels::<Fin>() as _;
        raw_config.formatOut = Sout::format() as _;
        raw_config.channelsOut = Sout::channels::<Fout>() as _;
        DataConverterConfig(raw_config, PhantomData)
    }
}
