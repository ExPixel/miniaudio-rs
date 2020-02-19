mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::*;
pub use types::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_struct_sizes() {
        assert_eq!(
            unsafe { debug_ma_sizeof_format_converter_config() },
            std::mem::size_of::<FormatConverterConfig>()
        );

        assert_eq!(
            unsafe { debug_ma_sizeof_format_converter() },
            std::mem::size_of::<FormatConverter>()
        );

        assert_eq!(
            unsafe { debug_ma_sizeof_channel_router_config() },
            std::mem::size_of::<ChannelRouterConfig>()
        );

        assert_eq!(
            unsafe { debug_ma_sizeof_channel_router() },
            std::mem::size_of::<ChannelRouter>()
        );

        assert_eq!(
            unsafe { debug_ma_sizeof_src_config_sinc() },
            std::mem::size_of::<SrcConfigSinc>()
        );

        assert_eq!(
            unsafe { debug_ma_sizeof_src_config() },
            std::mem::size_of::<SrcConfig>()
        );

        assert_eq!(unsafe { debug_ma_sizeof_src() }, std::mem::size_of::<Src>());

        assert_eq!(
            unsafe { debug_ma_sizeof_pcm_converter_config() },
            std::mem::size_of::<PCMConverterConfig>()
        );

        assert_eq!(
            unsafe { debug_ma_sizeof_pcm_converter() },
            std::mem::size_of::<PCMConverter>()
        );

        #[cfg(not(feature = "ma-no-device-io"))]
        {
            assert_eq!(
                unsafe { debug_ma_sizeof_thread() },
                std::mem::size_of::<Thread>()
            );

            assert_eq!(
                unsafe { debug_ma_sizeof_mutex() },
                std::mem::size_of::<Mutex>()
            );

            assert_eq!(
                unsafe { debug_ma_sizeof_event() },
                std::mem::size_of::<Event>()
            );

            assert_eq!(
                unsafe { debug_ma_sizeof_semaphore() },
                std::mem::size_of::<Semaphore>()
            );
        }
    }

    #[test]
    fn test_format_converter_config_fields() {
        let mut config = FormatConverterConfig::default();
        unsafe { debug_ma_init_format_converter_config(&mut config) };
        assert_eq!(config.format_in, Format::S16);
        assert_eq!(config.format_out, Format::S24);
        assert_eq!(config.channels, 45);
        assert_eq!(config.stream_format_in, StreamFormat::PCM);
        assert_eq!(config.stream_format_out, StreamFormat::PCM);
        assert_eq!(config.dither_mode, DitherMode::Rectangle);
        assert_eq!(config.no_sse2(), false);
        assert_eq!(config.no_avx2(), false);
        assert_eq!(config.no_avx512(), true);
        assert_eq!(config.no_neon(), true);
        assert_eq!(config.on_read, None);
        assert_eq!(config.on_read_deinterleaved, None);
        assert!(config.user_data.is_null());
    }

    #[test]
    fn test_format_converter_fields() {
        let mut converter = FormatConverter::default();
        unsafe { debug_ma_init_format_converter(&mut converter) };

        assert_eq!(converter.use_sse2(), true);
        assert_eq!(converter.use_avx2(), true);
        assert_eq!(converter.use_avx512(), false);
        assert_eq!(converter.use_neon(), false);

        // Check the Config:
        assert_eq!(converter.config.format_in, Format::S16);
        assert_eq!(converter.config.format_out, Format::S24);
        assert_eq!(converter.config.channels, 45);
        assert_eq!(converter.config.stream_format_in, StreamFormat::PCM);
        assert_eq!(converter.config.stream_format_out, StreamFormat::PCM);
        assert_eq!(converter.config.dither_mode, DitherMode::Rectangle);
        assert_eq!(converter.config.no_sse2(), false);
        assert_eq!(converter.config.no_avx2(), false);
        assert_eq!(converter.config.no_avx512(), true);
        assert_eq!(converter.config.no_neon(), true);
        assert_eq!(converter.config.on_read, None);
        assert_eq!(converter.config.on_read_deinterleaved, None);
        assert!(converter.config.user_data.is_null());
    }

    #[test]
    fn test_channel_router_config_fields() {
        let mut config = ChannelRouterConfig::default();
        unsafe { debug_ma_init_channel_router_config(&mut config) };
        assert_eq!(config.channels_in, 23);
        assert_eq!(config.channels_out, 483);
        assert_eq!(config.mixing_mode, ChannelMixMode::CustomWeights);
        assert_eq!(config.no_sse2(), false);
        assert_eq!(config.no_avx2(), false);
        assert_eq!(config.no_avx512(), true);
        assert_eq!(config.no_neon(), true);
        assert_eq!(config.on_read_deinterleaved, None);
        assert!(config.user_data.is_null());
    }

    #[test]
    fn test_channel_router_fields() {
        let mut router = ChannelRouter::default();
        unsafe { debug_ma_init_channel_router(&mut router) };

        assert_eq!(router.is_passthrough(), true);
        assert_eq!(router.is_simple_shuffle(), false);
        assert_eq!(router.is_simple_mono_expansion(), false);
        assert_eq!(router.is_stereo_to_mono(), true);
        assert_eq!(router.use_sse2(), true);
        assert_eq!(router.use_avx2(), true);
        assert_eq!(router.use_avx512(), false);
        assert_eq!(router.use_neon(), false);

        // Check the Config:
        assert_eq!(router.config.channels_in, 23);
        assert_eq!(router.config.channels_out, 483);
        assert_eq!(router.config.mixing_mode, ChannelMixMode::CustomWeights);
        assert_eq!(router.config.no_sse2(), false);
        assert_eq!(router.config.no_avx2(), false);
        assert_eq!(router.config.no_avx512(), true);
        assert_eq!(router.config.no_neon(), true);
        assert_eq!(router.config.on_read_deinterleaved, None);
        assert!(router.config.user_data.is_null());
    }

    #[test]
    fn test_src_config_fields() {
        let mut config = SrcConfig::default();
        unsafe { debug_ma_init_src_config(&mut config) };
        assert_eq!(config.sample_rate_in, 55);
        assert_eq!(config.sample_rate_out, 8734);
        assert_eq!(config.channels, 66);
        assert_eq!(config.algorithm, SrcAlgorithm::Sinc);
        assert_eq!(config.never_consume_end_of_input(), true);
        assert_eq!(config.no_sse2(), false);
        assert_eq!(config.no_avx2(), false);
        assert_eq!(config.no_avx512(), true);
        assert_eq!(config.no_neon(), true);
        assert_eq!(config.on_read_deinterleaved, None);
        assert!(config.user_data.is_null());
        assert_eq!(
            config.sinc.window_function,
            SrcSincWindowFunction::Rectangular
        );
        assert_eq!(config.sinc.window_width, 88);
    }

    #[test]
    fn test_src_fields() {
        let mut src = Src::default();
        unsafe { debug_ma_init_src(&mut src) };

        unsafe {
            assert_eq!(src.sinc().time_in, 45.0);
            assert_eq!(src.sinc().input_frame_count, 345);
            assert_eq!(src.sinc().window_pos_in_samples, 857);
        }

        assert_eq!(src.is_end_of_input_loaded(), false);
        assert_eq!(src.use_sse2(), true);
        assert_eq!(src.use_avx2(), true);
        assert_eq!(src.use_avx512(), false);
        assert_eq!(src.use_neon(), false);

        // Check the Config:
        assert_eq!(src.config.sample_rate_in, 55);
        assert_eq!(src.config.sample_rate_out, 8734);
        assert_eq!(src.config.channels, 66);
        assert_eq!(src.config.algorithm, SrcAlgorithm::Sinc);
        assert_eq!(src.config.never_consume_end_of_input(), true);
        assert_eq!(src.config.no_sse2(), false);
        assert_eq!(src.config.no_avx2(), false);
        assert_eq!(src.config.no_avx512(), true);
        assert_eq!(src.config.no_neon(), true);
        assert_eq!(src.config.on_read_deinterleaved, None);
        assert!(src.config.user_data.is_null());
        assert_eq!(
            src.config.sinc.window_function,
            SrcSincWindowFunction::Rectangular
        );
        assert_eq!(src.config.sinc.window_width, 88);
    }
}
