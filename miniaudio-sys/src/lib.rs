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
}
