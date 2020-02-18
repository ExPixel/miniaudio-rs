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
    }

    #[test]
    fn test_format_converter_config_alignment() {
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
    fn test_format_converter_alignment() {
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
}
