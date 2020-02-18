extern "C" {
    // Debug functions defined in wrapper. Used for ensuring that the size and alignment of structs
    // are correct.
    pub(crate) fn debug_ma_sizeof_format_converter_config() -> usize;
    pub(crate) fn debug_ma_sizeof_format_converter() -> usize;
    pub(crate) fn debug_ma_sizeof_channel_router_config() -> usize;
    pub(crate) fn debug_ma_sizeof_channel_router() -> usize;
    pub(crate) fn debug_ma_sizeof_src_config_sinc() -> usize;
    pub(crate) fn debug_ma_sizeof_src_config() -> usize;
    pub(crate) fn debug_ma_sizeof_src() -> usize;
}
