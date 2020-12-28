use crate::base::{from_bool8, Channel, ChannelMixMode, Error, Format, MAX_CHANNELS};
use crate::frames::{Frames, FramesMut};
use miniaudio_sys as sys;

/// Configuration for `ChannelConverter`.
#[repr(transparent)]
pub struct ChannelConverterConfig(sys::ma_channel_converter_config);

impl ChannelConverterConfig {
    pub fn new(
        format: Format,
        channel_map_in: &[Channel],
        channel_map_out: &[Channel],
        mixing_mode: ChannelMixMode,
    ) -> ChannelConverterConfig {
        ChannelConverterConfig(unsafe {
            sys::ma_channel_converter_config_init(
                format as _,
                channel_map_in.len() as u32,
                channel_map_in.as_ptr().cast(),
                channel_map_out.len() as u32,
                channel_map_out.as_ptr().cast(),
                mixing_mode as _,
            )
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn channels_in(&self) -> u32 {
        self.0.channelsIn
    }

    #[inline]
    pub fn channels_out(&self) -> u32 {
        self.0.channelsOut
    }

    #[inline]
    pub fn channel_map_in(&self) -> &[Channel] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.channelMapIn.as_ptr().cast(),
                self.0.channelsIn as usize,
            )
        }
    }

    #[inline]
    pub fn channel_map_out(&self) -> &[Channel] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.channelMapOut.as_ptr().cast(),
                self.0.channelsOut as usize,
            )
        }
    }

    #[inline]
    pub fn mixing_mode(&self) -> ChannelMixMode {
        ChannelMixMode::from_c(self.0.mixingMode)
    }

    /// Returns the weight for an in/out channel mapping.
    ///
    /// These weights are only used when mixing mode is set to `ChannelMixMode::CustomWeights`.
    #[inline]
    pub fn weight(&self, channel_in_index: usize, channel_out_index: usize) -> f32 {
        assert!(
            channel_in_index < self.0.channelsIn as usize
                && channel_out_index < self.0.channelsOut as usize,
            "channel in/out index out of bounds"
        );

        self.0.weights[channel_in_index][channel_out_index]
    }

    /// Set the weight for an in/out channel mapping.
    ///
    /// These weights are only used when mixing mode is set to `ChannelMixMode::CustomWeights`.
    #[inline]
    pub fn set_weight(&mut self, channel_in_index: usize, channel_out_index: usize, weight: f32) {
        assert!(
            channel_in_index < self.0.channelsIn as usize
                && channel_out_index < self.0.channelsOut as usize,
            "channel in/out index out of bounds"
        );

        self.0.weights[channel_in_index][channel_out_index] = weight;
    }
}

// FIXME For now this can use the default clone implementation because as far as I can tell
// ma_channel_converter_uninit does nothing, and there are no allocations to clean up. This may
// change in the future though so I should figure out a better cloning method.

/// channel conversion is used for channel rearrangement and conversion from one channel count to
/// another. The ChannelConverter API is used for channel conversion.
///
/// In addition to converting from one channel count to another the channel converter can also be
/// used to rearrange channels. When initializing the channel converter, you can optionally pass in
/// channel maps for both the input and output frames. If the channel counts are the same, each
/// channel map contains teh same channel positions with the exception that they're in a different
/// order, a simple shuffling of the channels will be performed. If, however, there is not a 1:1
/// mapping of channel positions, or the channel counts differ, the input channels will be mixed
/// based on a mixing mode which is specified when initializing the ChannelConverterConfig object.
///
/// When converting from mono to multi-channel, the mono channel is simply copied to each output
/// channel. When going the other way around, the audio of each output channel is simply averaged
/// and copied to the mono channel.
///
/// In more complicated cases blending is used. `ChannelMixMode::Simple` mode will drop excess
/// channels and silence extra channels. For example, convertion from 4 to 2 channels, the 3rd and
/// 4th channels will be dropped, whereas converting from 2 to 4 channels will put silence into the
/// 3rd and 4th channels.
///
/// `ChannelModeMode::Rectangle` mode uses spacial locality based on a rectangle to compute a
/// simple distribution between input and output. Imaging sitting in the middle of a root, with
/// speakers on the walls representing channel positions. Channel::FrontLeft position can be
/// thought of as being in the corder of the front and left walls.
///
/// Finally, `ChannelMixMode::CustomWeights` mode can be used to use custom user-defined weights.
#[repr(transparent)]
#[derive(Clone)]
pub struct ChannelConverter(sys::ma_channel_converter);

impl ChannelConverter {
    pub fn new(config: &ChannelConverterConfig) -> Result<ChannelConverter, Error> {
        let mut converter = std::mem::MaybeUninit::<ChannelConverter>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_channel_converter_init(
                &config.0 as *const _,
                converter.as_mut_ptr().cast(),
            ))?;
            Ok(converter.assume_init())
        }
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn channels_in(&self) -> u32 {
        self.0.channelsIn
    }

    #[inline]
    pub fn channels_out(&self) -> u32 {
        self.0.channelsOut
    }

    #[inline]
    pub fn channel_map_in(&self) -> &[Channel] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.channelMapIn.as_ptr().cast(),
                self.0.channelsIn as usize,
            )
        }
    }

    #[inline]
    pub fn channel_map_out(&self) -> &[Channel] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.channelMapOut.as_ptr().cast(),
                self.0.channelsOut as usize,
            )
        }
    }

    #[inline]
    pub fn mixing_mode(&self) -> ChannelMixMode {
        ChannelMixMode::from_c(self.0.mixingMode)
    }

    #[inline]
    pub fn is_passthrough(&self) -> bool {
        from_bool8(self.0.isPassthrough)
    }

    #[inline]
    pub fn is_simple_shuffle(&self) -> bool {
        from_bool8(self.0.isSimpleShuffle)
    }

    #[inline]
    pub fn is_simple_mono_expansion(&self) -> bool {
        from_bool8(self.0.isSimpleMonoExpansion)
    }

    #[inline]
    pub fn is_stereo_to_mono(&self) -> bool {
        from_bool8(self.0.isStereoToMono)
    }

    #[inline]
    pub fn shuffle_table(&self) -> &[u8; MAX_CHANNELS] {
        unsafe { std::mem::transmute(&self.0.shuffleTable) }
    }

    #[inline]
    pub fn process_pcm_frames(
        &mut self,
        output: &mut FramesMut,
        input: &Frames,
    ) -> Result<(), Error> {
        if output.format() != input.format() {
            ma_debug_panic!(
                "output and input format did not match (output: {:?}, input: {:?}",
                output.format(),
                input.format()
            );
            return Err(Error::InvalidArgs);
        }

        if output.frame_count() != input.frame_count() {
            ma_debug_panic!("output and input buffers did not have the same frame count (output: {}, input: {})", output.frame_count(), input.frame_count());
            return Err(Error::InvalidArgs);
        }

        Error::from_c_result(unsafe {
            sys::ma_channel_converter_process_pcm_frames(
                &mut self.0,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }
}

impl Drop for ChannelConverter {
    fn drop(&mut self) {
        unsafe {
            sys::ma_channel_converter_uninit(&mut self.0);
        }
    }
}
