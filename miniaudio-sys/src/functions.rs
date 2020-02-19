use super::constants::*;
use super::types::*;
use libc::{c_char, c_void};

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

    #[cfg(not(feature = "ma-no-device-io"))]
    pub fn debug_ma_sizeof_thread() -> usize;
    #[cfg(not(feature = "ma-no-device-io"))]
    pub fn debug_ma_sizeof_mutex() -> usize;
    #[cfg(not(feature = "ma-no-device-io"))]
    pub fn debug_ma_sizeof_event() -> usize;
    #[cfg(not(feature = "ma-no-device-io"))]
    pub fn debug_ma_sizeof_semaphore() -> usize;

    pub fn debug_ma_init_format_converter_config(config: &mut FormatConverterConfig);
    pub fn debug_ma_init_format_converter(converter: &mut FormatConverter);
    pub fn debug_ma_init_channel_router_config(config: &mut ChannelRouterConfig);
    pub fn debug_ma_init_channel_router(router: &mut ChannelRouter);
    pub fn debug_ma_init_src_config(config: &mut SrcConfig);
    pub fn debug_ma_init_src(src: &mut Src);
}

/*
 * Channel Maps
 */
extern "C" {
    /// Helper for retrieving a standard channel map.
    pub fn ma_get_standard_channel_map(
        standard_channel_map: StandardChannelMap,
        channels: u32,
        channel_map: &mut [Channel; MA_MAX_CHANNELS],
    );

    /// Copies a channel map.
    pub fn ma_channel_map_copy(ptr_out: *mut Channel, ptr_in: *const Channel, channels: u32);

    /// Determines whether or not a channel map is valid.
    ///
    /// A Blank channel map is valid (all channels set to MA_CHANNEL_NONE). The way a blank channel
    /// map is handled is context specific, but is usually treated as passthrough.
    ///
    /// Invalid Channels Maps:
    /// - A channel map with no channels.
    /// - A Channel map with more than one channel and a mono channel.
    pub fn ma_channel_map_valid(channels: u32, channelMap: &[Channel; MA_MAX_CHANNELS]) -> Bool;

    /// Helper for comparing two channel maps for equality.
    ///
    /// This assumes the channel count is the same between the two.
    pub fn ma_channel_map_equal(
        channels: u32,
        channelMapA: &[Channel; MA_MAX_CHANNELS],
        channelMapB: &[Channel; MA_MAX_CHANNELS],
    ) -> Bool;

    /// Gelper for determining if a channel map is blank (all channels set to MA_CHANNEL_NONE)
    pub fn ma_channel_map_blank(channels: u32, channelMap: &[Channel; MA_MAX_CHANNELS]) -> Bool;

    /// Helper for determining whether or not a channel is present in the given channel map.
    pub fn ma_channel_map_contains_channel_position(
        channels: u32,
        channelMap: &[Channel; MA_MAX_CHANNELS],
        channel_position: Channel,
    ) -> Bool;
}

/*
 * Format Conversion
 */
extern "C" {
    /// Initializes a format converter.
    #[must_use]
    pub fn ma_format_converter_init(
        config: &FormatConverterConfig,
        converter: &mut FormatConverter,
    ) -> Result;

    /// Reads data from the format converter as interleaved channels.
    pub fn ma_format_converter_read(
        converter: &mut FormatConverter,
        frame_count: u64,
        frames_out: *mut c_void,
        user_data: *mut c_void,
    ) -> u64;

    /// Reads data from the format converter as deinterleaved channels.
    pub fn ma_format_converter_read_deinterleaved(
        converter: &mut FormatConverter,
        frame_count: u64,
        samples_out: *mut *mut c_void,
        user_data: *mut c_void,
    ) -> u64;

    /// Helper for initializing a format converter config.
    pub fn ma_format_converter_config_init_new() -> FormatConverterConfig;

    /// Helper for initializing a format converter config with interleaved channels.
    pub fn ma_format_converter_config_init(
        format_in: Format,
        fotmat_out: Format,
        channels: u32,
        on_read: Option<FormatConverterReadProc>,
        user_data: *mut c_void,
    ) -> FormatConverterConfig;

    /// Helper for initializing a format converter config with deinterleaved channels.
    pub fn ma_format_converter_config_init_deinterleaved(
        format_in: Format,
        fotmat_out: Format,
        channels: u32,
        on_read_deinterleaved: Option<FormatConverterReadDeinterleavedProc>,
        user_data: *mut c_void,
    ) -> FormatConverterConfig;
}

/*
 * Channel Routing
 */
extern "C" {
    /// Initializes a channel router where it is assumed that the input data is non-interleaved.
    #[must_use]
    pub fn ma_channel_router_init(
        config: &ChannelRouterConfig,
        router: &mut ChannelRouter,
    ) -> Result;

    /// Reads data from the channel router as deinterleaved channels.
    pub fn ma_channel_router_read_deinterleaved(
        router: &mut ChannelRouter,
        frame_count: u64,
        samples_out: *mut *mut c_void,
        user_data: *mut c_void,
    ) -> u64;

    /// Helper for initializing a channel router config.
    pub fn ma_channel_router_config_init(
        channels_in: u32,
        channel_map_in: &[Channel; MA_MAX_CHANNELS],
        channels_out: u32,
        channel_map_out: &[Channel; MA_MAX_CHANNELS],
        mixing_mode: ChannelMixMode,
        on_read: Option<ChannelRouterReadDeinterleavedProc>,
        user_data: *mut c_void,
    ) -> ChannelRouterConfig;
}

/*
 * Sample Rate Conversion
 */
extern "C" {
    /// Initializes a sample rate conversion object.
    #[must_use]
    pub fn ma_src_init(config: &SrcConfig, src: &mut Src) -> Result;

    /// Dynamically adjusts the sample rate.
    ///
    /// This is useful for dynamically adjusting the pitch. Keep in mind, however, that this will
    /// speed up or slow down the sound. If this is not acceptable you will need to use your own
    /// algorithm.
    #[must_use]
    pub fn ma_src_set_sample_rate(
        src: &mut Src,
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> Result;

    /// Reads a number of frames.
    ///
    /// Returns the number of frames actually read.
    pub fn ma_src_read_deinterleaved(
        src: &mut Src,
        frame_couter: u64,
        samples_out: *mut *mut c_void,
        user_data: *mut c_void,
    ) -> u64;

    /// Helper for creating a sample rate conversion config.
    pub fn ma_src_config_init_new() -> SrcConfig;

    /// Helper for creating a sample rate conversion config.
    pub fn ma_src_config_init(
        sample_rate_in: u32,
        sample_rate_out: u32,
        channels: u32,
        on_read_deinterleaved: Option<SrcReadDeinterleavedProc>,
        user_data: c_void,
    ) -> SrcConfig;
}

/*
 * Conversion
 */
extern "C" {
    /// Initializes a DSP object.
    #[must_use]
    pub fn ma_pcm_converter_init(config: &PCMConverterConfig, dsp: &mut PCMConverter) -> Result;

    /// Dynamically adjusts the input sample rate.
    ///
    /// This will fail if the DSP was not initialized with `allowDynamicSampleRate`
    ///
    /// **[DEPRECATED]** Use [ma_pcm_converter_set_sample_rate](fn.ma_src_set_sample_rate.html) instead.
    #[deprecated]
    #[must_use]
    pub fn ma_pcm_converter_set_input_sample_rate(
        dsp: &mut PCMConverter,
        sample_rate_out: u32,
    ) -> Result;

    /// Dynamically adjusts the output sample rate.
    ///
    /// This is useful for dynamically adjust pitch. Keep in mind, however, that this will speed up or slow down the sound. If this
    /// is not acceptable you will need to use your own algorithm.
    ///
    /// This will fail is the DSP was not initialized with allowDynamicSampleRate.
    ///
    /// **[DEPRECATED]** Use [ma_pcm_converter_set_sample_rate](fn.ma_src_set_sample_rate.html) instead.
    #[must_use]
    pub fn ma_pcm_converter_set_output_sample_rate(
        dsp: &mut PCMConverter,
        sample_rate_out: u32,
    ) -> Result;

    /// Dynamically adjusts the output sample rate.
    ///
    /// This is useful for dynamically adjust pitch. Keep in mind, however, that this will speed up or slow down the sound. If this
    /// is not acceptable you will need to use your own algorithm.
    ///
    /// This will fail if the DSP was not initialized with allowDynamicSampleRate.
    #[must_use]
    pub fn ma_pcm_converter_set_sample_rate(
        dsp: &mut PCMConverter,
        sample_rate_in: u32,
        sample_rate_out: u32,
    ) -> Result;

    /// Reads a number of frames and runs them through the DSP processor.
    pub fn ma_pcm_converter_read(
        dsp: &mut PCMConverter,
        frames_out: *mut c_void,
        frame_count: u64,
    ) -> u64;

    /// Helper for initializing a PCMConverterConfig object.
    pub fn ma_pcm_converter_config_init_new() -> PCMConverterConfig;

    /// Helper for initializing a PCMConverterConfig object.
    pub fn ma_pcm_converter_config_init(
        format_in: Format,
        channels_in: u32,
        sample_rate_in: u32,
        format_out: Format,
        channels_out: u32,
        sample_rate_out: u32,
        on_read: Option<PCMConverterReadProc>,
        user_data: *mut c_void,
    ) -> PCMConverterConfig;

    /// Helper for initializing a PCMConverterConfig object.
    pub fn ma_pcm_converter_config_init_ex(
        format_in: Format,
        channels_in: u32,
        sample_rate_in: u32,
        channel_map_in: &mut [Channel; MA_MAX_CHANNELS],
        format_out: Format,
        channels_out: u32,
        sample_rate_out: u32,
        channel_map_out: &mut [Channel; MA_MAX_CHANNELS],
        on_read: Option<PCMConverterReadProc>,
        user_data: *mut c_void,
    ) -> PCMConverterConfig;

    /// High-level helper for doing a full format conversion in one go. Returns the number of
    /// output frames. Call this with `ptr_out` set to NULL to determine the required size of teh output
    /// buffer.
    ///
    /// A return value of 0 indicates an error.
    ///
    /// This function is useful for one-off bulk conversions, but if you're streaming data you
    /// should use the `ma_pcm_converter` APIs instead.
    pub fn ma_convert_frames(
        ptr_out: *mut c_void,
        format_out: Format,
        channels_out: u32,
        sample_rate_out: u32,
        ptr_in: *const c_void,
        format_in: Format,
        channels_in: u32,
        sample_rate_in: u32,
        frame_count: u64,
    ) -> u64;

    /// High-level helper for doing a full format conversion in one go. Returns the number of
    /// output frames. Call this with `ptr_out` set to NULL to determine the required size of teh output
    /// buffer.
    ///
    /// A return value of 0 indicates an error.
    ///
    /// This function is useful for one-off bulk conversions, but if you're streaming data you
    /// should use the `ma_pcm_converter` APIs instead.
    pub fn ma_convert_frames_ex(
        ptr_out: *mut c_void,
        format_out: Format,
        channels_out: u32,
        sample_rate_out: u32,
        channel_map_out: &mut [Channel; MA_MAX_CHANNELS],
        ptr_in: *const c_void,
        format_in: Format,
        channels_in: u32,
        sample_rate_in: u32,
        channel_map_in: &mut [Channel; MA_MAX_CHANNELS],
        frame_count: u64,
    ) -> u64;
}

/*
 * MISC
 */
extern "C" {
    /// Retrieves the size of a sample in bytes for the given format.
    ///
    /// This API is efficient and is implemented using a lookup table.
    pub fn ma_get_bytes_per_sample(format: Format) -> u32;

    /// Converts a log level to a string.
    pub fn ma_log_level_to_string(log_level: LogLevel) -> *const c_char;
}

// While this is really implemented in Rust, it's still part of the original C API so in case
// something changes and it's not longer just an inlined function I won't break API compatibility
// by changing it into a foreign function if it is one from the beginning.
// Still marked as inlined though so that it can be inlined across crates.
#[inline]
pub unsafe extern "C" fn ma_get_bytes_per_frame(format: Format, channels: u32) -> u32 {
    return ma_get_bytes_per_sample(format) * channels;
}

/*
 * Format Conversion
 */
extern "C" {
    pub fn ma_pcm_u8_to_s16(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_u8_to_s24(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_u8_to_s32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_u8_to_f32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s16_to_u8(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s16_to_s24(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s16_to_s32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s16_to_f32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s24_to_u8(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s24_to_s16(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s24_to_s32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s24_to_f32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s32_to_u8(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s32_to_s16(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s32_to_s24(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_s32_to_f32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_f32_to_u8(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_f32_to_s16(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_f32_to_s24(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );
    pub fn ma_pcm_f32_to_s32(
        ptr_out: *mut c_void,
        ptr_in: *const c_void,
        count: u64,
        dither_mode: DitherMode,
    );

    pub fn ma_pcm_convert(
        ptr_out: *mut c_void,
        format_out: Format,
        ptr_int: *const c_void,
        format_in: Format,
        sample_count: u64,
        dither_mode: DitherMode,
    );

    /// Deinterleaves an interleaved buffer.
    pub fn ma_deinterleave_pcm_frames(
        format: Format,
        channels: u32,
        frame_count: u64,
        interleaved_pcm_frames_ptr: *const c_void,
        deinterleaved_pcm_frames_ptr: *mut *mut c_void,
    );

    pub fn ma_interleave_pcm_frames(
        format: Format,
        channels: u32,
        frame_count: u32,
        deinterleaved_pcm_frames_ptr: *const *const c_void,
        interleaved_pcm_frames: *mut c_void,
    );
}

/*
 * Device IO
 */
extern "C" {}
