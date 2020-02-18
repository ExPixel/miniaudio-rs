use super::types::*;

/// This is here temporarily to silence warning about nothing exportable
/// in this module.
pub const SILENCE_WARNINGS: Channel = Channel::Aux0;

#[cfg(test)]
extern "C" {
    // Debug functions defined in wrapper. Used for ensuring that the size and alignment of structs
    // are correct.
    pub fn debug_ma_sizeof_format_converter_config() -> usize;
    pub fn debug_ma_sizeof_format_converter() -> usize;
    pub fn debug_ma_sizeof_channel_router_config() -> usize;
    pub fn debug_ma_sizeof_channel_router() -> usize;
    pub fn debug_ma_sizeof_src_config_sinc() -> usize;
    pub fn debug_ma_sizeof_src_config() -> usize;
    pub fn debug_ma_sizeof_src() -> usize;
    pub fn debug_ma_sizeof_pcm_converter_config() -> usize;
    pub fn debug_ma_sizeof_pcm_converter() -> usize;
    pub fn debug_ma_init_format_converter_config(config: &mut FormatConverterConfig);
    pub fn debug_ma_init_format_converter(converter: &mut FormatConverter);
    pub fn debug_ma_init_channel_router_config(config: &mut ChannelRouterConfig);
    pub fn debug_ma_init_channel_router(router: &mut ChannelRouter);
    pub fn debug_ma_init_src_config(config: &mut SrcConfig);
    pub fn debug_ma_init_src(src: &mut Src);
}
