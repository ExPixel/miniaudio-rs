use super::constants::*;

#[cfg(not(feature = "ma-no-device-io"))]
pub use device_io::*;

use libc::{c_float, c_void};

macro_rules! impl_void_debug {
    ($Type:ty, $Name:expr) => {
        impl std::fmt::Debug for $Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!($Name, "{{ /* omitted */ }}"))
            }
        }
    };

    ($Type:ty) => {
        impl_void_debug!($Type, stringify!($Type));
    };
}

macro_rules! impl_default {
    ($Type:ty, $Value:expr) => {
        impl Default for $Type {
            fn default() -> $Type {
                $Value
            }
        }
    };
}

pub type Handle = *mut c_void;
pub type Proc = extern "C" fn();
pub type Ptr = *mut c_void;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum LogLevel {
    Error = 1,
    Warning = 2,
    Info = 3,
    Verbose = 4,
}

bitflags::bitflags! {
    /// 32-bit boolean value used by mini-audio.
    #[repr(transparent)]
    pub struct Bool: u32 {
        const FALSE = 0;
        const TRUE = 1;
    }
}

impl_default!(Bool, Bool::FALSE);

impl From<Bool> for bool {
    #[inline]
    fn from(b: Bool) -> bool {
        b != Bool::FALSE
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(b: bool) -> Bool {
        if b {
            Bool::TRUE
        } else {
            Bool::FALSE
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Result {
    Success = 0,

    /* General errors. */
    /// A generic error.
    Error = -1,
    InvalidArgs = -2,
    InvalidOperation = -3,
    OutOfMemory = -4,
    AccessDenied = -5,
    TooLarge = -6,
    Timeout = -7,

    /* General miniaudio-specific errors. */
    FormatNotSupported = -100,
    DeviceTypeNotSupported = -101,
    ShareModeNotSupported = -102,
    NoBackend = -103,
    NoDevice = -104,
    ApiNotFound = -105,
    InvalidDeviceConfig = -106,

    /* State errors. */
    DeviceBusy = -200,
    DeviceNotInitialized = -201,
    DeviceNotStarted = -202,
    DeviceUnavailable = -203,

    /* Operation errors. */
    FailedToMapDeviceBuffer = -300,
    FailedToUnmapDeviceBuffer = -301,
    FailedToInitBackend = -302,
    FailedToReadDataFromClient = -303,
    FailedToReadDataFromDevice = -304,
    FailedToSendDataToClient = -305,
    FailedToSendDataToDevice = -306,
    FailedToOpenBackendDevice = -307,
    FailedToStartBackendDevice = -308,
    FailedToStopBackendDevice = -309,
    FailedToConfigureBackendDevice = -310,
    FailedToCreateMutex = -311,
    FailedToCreateEvent = -312,
    FailedToCreateSemaphore = -313,
    FailedToCreateThread = -314,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Channel {
    None = 0,
    Mono = 1,
    FrontLeft = 2,
    FrontRight = 3,
    FrontCenter = 4,
    Lfe = 5,
    BackLeft = 6,
    BackRight = 7,
    FrontLeftCenter = 8,
    FrontRightCenter = 9,
    BackCenter = 10,
    SideLeft = 11,
    SideRight = 12,
    TopCenter = 13,
    TopFrontLeft = 14,
    TopFrontCenter = 15,
    TopFrontRight = 16,
    TopBackLeft = 17,
    TopBackCenter = 18,
    TopBackRight = 19,
    Aux0 = 20,
    Aux1 = 21,
    Aux2 = 22,
    Aux3 = 23,
    Aux4 = 24,
    Aux5 = 25,
    Aux6 = 26,
    Aux7 = 27,
    Aux8 = 28,
    Aux9 = 29,
    Aux10 = 30,
    Aux11 = 31,
    Aux12 = 32,
    Aux13 = 33,
    Aux14 = 34,
    Aux15 = 35,
    Aux16 = 36,
    Aux17 = 37,
    Aux18 = 38,
    Aux19 = 39,
    Aux20 = 40,
    Aux21 = 41,
    Aux22 = 42,
    Aux23 = 43,
    Aux24 = 44,
    Aux25 = 45,
    Aux26 = 46,
    Aux27 = 47,
    Aux28 = 48,
    Aux29 = 49,
    Aux30 = 50,
    Aux31 = 51,
}

impl Channel {
    #[inline]
    pub const fn left() -> Channel {
        Channel::FrontLeft
    }

    #[inline]
    pub const fn right() -> Channel {
        Channel::FrontRight
    }

    #[inline]
    pub const fn count() -> u8 {
        Channel::Aux31 as u8
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StreamFormat {
    PCM = 0,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StreamLayout {
    Interleaved = 0,
    Deinterleaved = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum DitherMode {
    None = 0,
    Rectangle = 1,
    Triangle = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Format {
    /// Mainly used for indicating error, but also used as the default for the output format for
    /// decoders.
    Unknown = 0,
    U8 = 1,
    /// Sems to be the most widely supported format.
    S16 = 2,
    S24 = 3,
    S32 = 4,
    F32 = 5,
}

impl Format {
    pub const COUNT: usize = 6;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum ChannelMixMode {
    /// Simple averaging based on the plane(s) the channel is sitting on.
    Rectangular = 0,
    /// Drop excess channels; zeroed out extra channels.
    Simple = 1,
    /// Use custom weights specified in ma_channel_router_config.
    CustomWeights = 2,
}

impl ChannelMixMode {
    #[inline]
    pub const fn planar_blend() -> ChannelMixMode {
        ChannelMixMode::Rectangular
    }
}

impl Default for ChannelMixMode {
    #[inline]
    fn default() -> ChannelMixMode {
        ChannelMixMode::planar_blend()
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StandardChannelMap {
    Microsoft = 0,
    Alsa = 1,
    /// Based on AIFF.
    RFC3551 = 2,
    Flac = 3,
    Vorbis = 4,
    /// FreeBSD's sound(4).
    Sound4 = 5,
    /// www.sndio.org/tips.html
    SNDIO = 6,
}

impl StandardChannelMap {
    /// https://webaudio.github.io/web-audio-api/#ChannelOrdering. Only 1, 2, 4 and 6 channels are
    #[inline]
    pub const fn web_audio() -> StandardChannelMap {
        StandardChannelMap::Flac
    }
}

impl Default for StandardChannelMap {
    #[inline]
    fn default() -> StandardChannelMap {
        StandardChannelMap::Microsoft
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum PerformanceProfile {
    LowLatency = 0,
    Conservative = 1,
}

pub type FormatConverterReadProc = extern "C" fn(
    converter: *mut FormatConverter,
    frame_count: u32,
    frames_out: *mut c_void,
    user_data: *mut c_void,
) -> u32;
pub type FormatConverterReadDeinterleavedProc = extern "C" fn(
    converter: *mut FormatConverter,
    frame_count: u32,
    pp_samples_out: *mut *mut c_void,
    user_data: *mut c_void,
) -> u32;

const NO_SSE2_MASK: u32 = 1 << 0;
const NO_AVX2_MASK: u32 = 1 << 1;
const NO_AVX512_MASK: u32 = 1 << 2;
const NO_NEON_MASK: u32 = 1 << 3;

const USE_SSE2_MASK: u32 = 1 << 0;
const USE_AVX2_MASK: u32 = 1 << 1;
const USE_AVX512_MASK: u32 = 1 << 2;
const USE_NEON_MASK: u32 = 1 << 3;

macro_rules! impl_bitfield {
    ($ForType:ty, $BitField:ident, $Set:ident, $Get:ident, $Mask:expr) => {
        impl $ForType {
            #[inline]
            pub fn $Set(&mut self, value: bool) {
                if value {
                    self.$BitField |= $Mask;
                } else {
                    self.$BitField &= !($Mask);
                }
            }

            #[inline]
            pub fn $Get(&self) -> bool {
                (self.$BitField & $Mask) != 0
            }
        }
    };

    ($ForType:ty, $BitField:ident, $Set:ident, $Get:ident, $Mask:expr, $Doc:expr) => {
        impl $ForType {
            #[doc = $Doc]
            #[inline]
            pub fn $Set(&mut self, value: bool) {
                if value {
                    self.$BitField |= $Mask;
                } else {
                    self.$BitField &= !($Mask);
                }
            }

            #[doc = $Doc]
            #[inline]
            pub fn $Get(&self) -> bool {
                (self.$BitField & $Mask) != 0
            }
        }
    };
}

macro_rules! impl_use_simd_bitfields {
    ($ForType:ty, $SIMDField:ident, $Offset:expr) => {
        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_sse2,
            use_sse2,
            USE_SSE2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_avx2,
            use_avx2,
            USE_AVX2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_avx512,
            use_avx512,
            USE_AVX512_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_neon,
            use_neon,
            USE_NEON_MASK << $Offset
        );
    };
}

macro_rules! impl_no_simd_bitfields {
    ($ForType:ty, $SIMDField:ident, $Offset:expr) => {
        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_sse2,
            no_sse2,
            NO_SSE2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_avx2,
            no_avx2,
            NO_AVX2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_avx512,
            no_avx512,
            NO_AVX512_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_neon,
            no_neon,
            NO_NEON_MASK << $Offset
        );
    };
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FormatConverterConfig {
    pub format_in: Format,
    pub format_out: Format,
    pub channels: u32,
    pub stream_format_in: StreamFormat,
    pub stream_format_out: StreamFormat,
    pub dither_mode: DitherMode,
    simd_bits: u32,
    pub on_read: Option<FormatConverterReadProc>,
    pub on_read_deinterleaved: Option<FormatConverterReadDeinterleavedProc>,
    pub user_data: *mut c_void,
}
impl_no_simd_bitfields!(FormatConverterConfig, simd_bits, 0);

impl Default for FormatConverterConfig {
    fn default() -> FormatConverterConfig {
        FormatConverterConfig {
            format_in: Format::Unknown,
            format_out: Format::Unknown,
            channels: 0,
            stream_format_in: StreamFormat::PCM,
            stream_format_out: StreamFormat::PCM,
            dither_mode: DitherMode::None,
            simd_bits: 0,
            on_read: None,
            on_read_deinterleaved: None,
            user_data: std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FormatConverter {
    pub config: FormatConverterConfig,
    pub simd_bits: u32,
    pub on_convert_pcm: Option<
        extern "C" fn(dst: *mut c_void, src: *const c_void, count: u64, dither_mode: DitherMode),
    >,
    pub on_interleave_pcm: Option<
        extern "C" fn(dst: *mut c_void, src: *const *const c_void, frame_count: u64, channels: u32),
    >,
    pub on_deinterleave_pcm: Option<
        extern "C" fn(dst: *mut *mut c_void, src: *const c_void, frame_count: u64, channels: u32),
    >,
}
impl_use_simd_bitfields!(FormatConverter, simd_bits, 0);

impl Default for FormatConverter {
    fn default() -> FormatConverter {
        FormatConverter {
            config: FormatConverterConfig::default(),
            simd_bits: 0,
            on_convert_pcm: None,
            on_interleave_pcm: None,
            on_deinterleave_pcm: None,
        }
    }
}

pub type ChannelRouterReadDeinterleavedProc = extern "C" fn(
    router: *mut ChannelRouter,
    frame_count: u32,
    pp_samples_out: *mut *mut c_void,
    user_data: *mut c_void,
) -> u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChannelRouterConfig {
    pub channels_in: u32,
    pub channels_out: u32,
    pub channel_map_in: [Channel; MA_MAX_CHANNELS],
    pub channel_map_out: [Channel; MA_MAX_CHANNELS],
    pub mixing_mode: ChannelMixMode,
    pub weights: [[c_float; MA_MAX_CHANNELS]; MA_MAX_CHANNELS],
    pub simd_bits: u32,
    pub on_read_deinterleaved: Option<ChannelRouterReadDeinterleavedProc>,
    pub user_data: *mut c_void,
}
impl_no_simd_bitfields!(ChannelRouterConfig, simd_bits, 0);

impl Default for ChannelRouterConfig {
    fn default() -> ChannelRouterConfig {
        ChannelRouterConfig {
            channels_in: 0,
            channels_out: 0,
            channel_map_in: [Channel::None; MA_MAX_CHANNELS],
            channel_map_out: [Channel::None; MA_MAX_CHANNELS],
            mixing_mode: ChannelMixMode::Rectangular,
            weights: [[0f32; MA_MAX_CHANNELS]; MA_MAX_CHANNELS],
            simd_bits: 0,
            on_read_deinterleaved: None,
            user_data: std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChannelRouter {
    pub config: ChannelRouterConfig,
    bitfields: u32,
    pub shuffle_table: [u8; MA_MAX_CHANNELS],
}

impl_bitfield!(
    ChannelRouter,
    bitfields,
    set_is_passthrough,
    is_passthrough,
    1 << 0
);
impl_bitfield!(
    ChannelRouter,
    bitfields,
    set_is_simple_shuffle,
    is_simple_shuffle,
    1 << 1
);
impl_bitfield!(
    ChannelRouter,
    bitfields,
    set_is_simple_mono_expansion,
    is_simple_mono_expansion,
    1 << 2
);
impl_bitfield!(
    ChannelRouter,
    bitfields,
    set_is_stereo_to_mono,
    is_stereo_to_mono,
    1 << 3
);
impl_use_simd_bitfields!(ChannelRouter, bitfields, 4);

impl Default for ChannelRouter {
    fn default() -> ChannelRouter {
        ChannelRouter {
            config: ChannelRouterConfig::default(),
            bitfields: 0,
            shuffle_table: [0u8; MA_MAX_CHANNELS],
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SrcAlgorithm {
    Linear = 0,
    Sinc = 1,
    None = 2,
}

impl Default for SrcAlgorithm {
    fn default() -> SrcAlgorithm {
        SrcAlgorithm::Linear
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SrcSincWindowFunction {
    Hann = 0,
    Rectangular = 1,
}

impl Default for SrcSincWindowFunction {
    fn default() -> SrcSincWindowFunction {
        SrcSincWindowFunction::Hann
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SrcConfigSinc {
    pub window_function: SrcSincWindowFunction,
    pub window_width: u32,
}

impl Default for SrcConfigSinc {
    fn default() -> SrcConfigSinc {
        SrcConfigSinc {
            window_function: SrcSincWindowFunction::default(),
            window_width: 0,
        }
    }
}

/// Returns the number of frames that were read.
pub type SrcReadDeinterleavedProc = extern "C" fn(
    p_src: *mut Src,
    frame_count: u32,
    pp_samples_out: *mut *mut c_void,
    user_data: *mut c_void,
) -> u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SrcConfig {
    pub sample_rate_in: u32,
    pub sample_rate_out: u32,
    pub channels: u32,
    pub algorithm: SrcAlgorithm,
    bitfields: u32,
    pub on_read_deinterleaved: Option<SrcReadDeinterleavedProc>,
    pub user_data: *mut c_void,
    pub sinc: SrcConfigSinc,
}
impl_bitfield!(
    SrcConfig,
    bitfields,
    set_never_consume_end_of_input,
    never_consume_end_of_input,
    1 << 0
);
impl_no_simd_bitfields!(SrcConfig, bitfields, 1);

impl Default for SrcConfig {
    fn default() -> SrcConfig {
        SrcConfig {
            sample_rate_in: 0,
            sample_rate_out: 0,
            channels: 0,
            algorithm: SrcAlgorithm::default(),
            bitfields: 0,
            on_read_deinterleaved: None,
            user_data: std::ptr::null_mut(),
            sinc: SrcConfigSinc::default(),
        }
    }
}

#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Src {
    inner: SrcInnerUnion,
    pub config: SrcConfig,
    bitfields: u32,
}
impl_bitfield!(
    Src,
    bitfields,
    set_is_end_of_input_loaded,
    is_end_of_input_loaded,
    1 << 0
);
impl_use_simd_bitfields!(Src, bitfields, 1);

impl Src {
    #[inline]
    pub unsafe fn linear(&self) -> &SrcLinear {
        &self.inner.linear
    }

    #[inline]
    pub unsafe fn sinc(&self) -> &SrcSinc {
        &self.inner.sinc
    }

    #[inline]
    pub fn set_linear(&mut self, linear: SrcLinear) {
        self.inner.linear = linear;
    }

    #[inline]
    pub fn set_sinc(&mut self, sinc: SrcSinc) {
        self.inner.sinc = sinc;
    }
}

impl Default for Src {
    fn default() -> Src {
        Src {
            inner: SrcInnerUnion {
                linear: SrcLinear {
                    input: [[0.0f32; MA_MAX_CHANNELS]; MA_SRC_INPUT_BUFFER_SIZE_IN_SAMPLES],
                    time_in: 0.0f32,
                    left_over_frames: 0,
                },
            },

            config: SrcConfig::default(),
            bitfields: 0,
        }
    }
}

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub union SrcInnerUnion {
    pub linear: SrcLinear,
    pub sinc: SrcSinc,
}

// FIXME: implement better debug output for this type.
impl_void_debug!(SrcInnerUnion);

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct SrcLinear {
    pub input: [[c_float; MA_MAX_CHANNELS]; MA_SRC_INPUT_BUFFER_SIZE_IN_SAMPLES],
    pub time_in: c_float,
    pub left_over_frames: u32,
}

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct SrcSinc {
    pub input: [[c_float; MA_MAX_CHANNELS];
        MA_SRC_SINC_MAX_WINDOW_WIDTH * 2 + MA_SRC_INPUT_BUFFER_SIZE_IN_SAMPLES],
    pub time_in: c_float,
    /// The number of frames sitting in the input buffer, not including the first half of the
    /// window.
    pub input_frame_count: u32,
    /// An offset of `input`.
    pub window_pos_in_samples: u32,
    /// Precomputed lookup table. The +1 is used to avoid the need for an overflow check.
    pub table: [c_float; MA_SRC_SINC_MAX_WINDOW_WIDTH * MA_SRC_SINC_LOOKUP_TABLE_RESOLUTION],
}

pub type PCMConverterReadProc = extern "C" fn(
    dsp: *mut PCMConverter,
    frames_out: *mut c_void,
    frame_count: u32,
    user_data: *mut c_void,
) -> u32;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum DynamicSampleRate {
    Disallow = 0,
    Allow = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PCMConverterConfig {
    pub format_in: Format,
    pub channels_in: u32,
    pub sample_rate_in: u32,
    pub channel_map_in: [Channel; MA_MAX_CHANNELS],
    pub format_out: Format,
    pub channels_out: u32,
    pub sample_rate_out: u32,
    pub channel_map_out: [Channel; MA_MAX_CHANNELS],
    pub channel_mix_mode: ChannelMixMode,
    pub dither_mode: DitherMode,
    pub src_algorithm: SrcAlgorithm,
    pub allow_dynamic_sample_rate: DynamicSampleRate,
    bitfields: u32,
    pub on_read: Option<PCMConverterReadProc>,
    pub user_data: *mut c_void,
    sinc_union: PCMConverterSinc,
}
impl_bitfield!(
    PCMConverterConfig,
    bitfields,
    set_never_consume_end_of_input,
    never_consume_end_of_input,
    1 << 0
);
impl_no_simd_bitfields!(PCMConverterConfig, bitfields, 1);

impl PCMConverterConfig {
    #[inline]
    pub unsafe fn sinc(&self) -> &SrcConfigSinc {
        &self.sinc_union.sinc
    }

    pub fn set_sinc(&mut self, sinc: SrcConfigSinc) {
        self.sinc_union.sinc = sinc;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union PCMConverterSinc {
    pub sinc: SrcConfigSinc,
}
impl_void_debug!(PCMConverterSinc);

#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct PCMConverter {
    pub on_read: Option<PCMConverterReadProc>,
    pub user_data: *mut c_void,

    /// For converting data to f32 in preparation for further processing.
    pub format_converter_in: FormatConverter,
    /// For converting data to the requested output format. USed as the final step in the
    /// processing pipeline.
    pub format_converter_out: FormatConverter,
    /// For channel conversion.
    pub channel_router: ChannelRouter,
    /// For sample rate conversion.
    pub src: Src,
    bitfields: u32,
}

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_dynamic_sample_rate_allowed,
    is_dynamic_sample_rate_allowed,
    1 << 0,
    "ma_pcm_converset_set_input_sample_rate() and ma_pcm_set_output_sample_rate() will fail if this is set to false."
);

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_pre_format_conversion_required,
    is_pre_format_conversion_required,
    1 << 1
);

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_post_format_conversion_required,
    is_post_format_conversion_required,
    1 << 2
);

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_channel_routing_required,
    is_channel_routing_required,
    1 << 3
);

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_src_required,
    is_src_required,
    1 << 4
);

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_channel_routing_at_start,
    is_channel_routing_at_start,
    1 << 5
);

impl_bitfield!(
    PCMConverter,
    bitfields,
    set_is_passthrough,
    is_passthrough,
    1 << 6,
    "Will be set to true when the conversion pipeline is an optimization passthrough."
);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RingBuffer {
    pub buffer: *mut libc::c_void,
    pub subbuffer_size_in_bytes: u32,
    pub subbuffer_count: u32,
    pub subbuffer_stride_in_bytes: u32,

    /// Mose significant bit is the loop flag. Lower 31 bits contains the actual offset in bytes.
    encoded_read_offset: u32,
    /// Most significant bit is the loop flag. Lower 31 bits contains the actual offset in bytes.
    encoded_write_offset: u32,

    bitfields: u32,
}

impl RingBuffer {
    /// Does a volatile read of the internal `encoded_read_offset`.
    #[inline]
    pub fn encoded_read_offset(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.encoded_read_offset) }
    }

    /// Does a volatile read of the internal `encoded_write_offset`.
    #[inline]
    pub fn encoded_write_offset(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.encoded_write_offset) }
    }

    /// Does a volatile write to the internal `encoded_read_offset`.
    #[inline]
    pub fn set_encoded_read_offset(&mut self, value: u32) {
        unsafe { std::ptr::write_volatile(&mut self.encoded_read_offset, value) }
    }

    /// Does a volatile write to the internal `encoded_write_offset`.
    #[inline]
    pub fn set_encoded_write_offset(&mut self, value: u32) {
        unsafe { std::ptr::write_volatile(&mut self.encoded_write_offset, value) }
    }
}

impl_bitfield!(RingBuffer, bitfields, set_owns_buffer, owns_buffer, 1 << 0);
impl_bitfield!(
    RingBuffer,
    bitfields,
    set_clear_on_write_acquire,
    clear_on_write_acquire,
    1 << 1
);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PCMRingBuffer {
    pub ring_buffer: RingBuffer,
    pub format: Format,
    pub channels: u32,
}

#[cfg(not(feature = "ma-no-device-io"))]
mod device_io {
    use super::*;
    use libc::{c_char, c_int, c_void};

    #[cfg(feature = "ma-support-wasapi")]
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct IMMNotificationClient {
        lp_vtbl: *mut c_void,
        counter: u32,
        device: *mut Device,
    }

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum Backend {
        WASAPI = 0,
        DSOUND = 1,
        WINMM = 2,
        CoreAudio = 3,
        SNDIO = 4,
        Audio4 = 5,
        OSS = 6,
        PulseAudio = 7,
        Alsa = 8,
        Jack = 9,
        AAudio = 10,
        OpenSL = 11,
        WebAudio = 12,

        // Must always be the last item. Lowest priority, and used as the terminator for backend
        // enumeration.
        Null = 13,
    }

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum ThreadPriority {
        Idle = -5,
        Lowest = -4,
        Low = -3,
        Normal = -2,
        High = -1,
        Highest = 0,
        Realtime = 1,
    }

    impl Default for ThreadPriority {
        fn default() -> ThreadPriority {
            ThreadPriority::Highest
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Thread {
        pub context: *mut Context,
        pub platform: PlatformThread,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union PlatformThread {
        #[cfg(feature = "ma-win32")]
        pub win32: Win32Thread,
        #[cfg(feature = "ma-posix")]
        pub posix: PosixThread,
        pub unused: c_int,
    }
    impl_void_debug!(PlatformThread);

    #[cfg(feature = "ma-win32")]
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Win32Thread {
        pub thread_handle: Handle,
    }
    #[cfg(feature = "ma-win32")]
    impl_void_debug!(Win32Thread);

    #[cfg(feature = "ma-posix")]
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct PosixThread {
        pub thread: libc::pthread_t,
    }
    #[cfg(feature = "ma-posix")]
    impl_void_debug!(PosixThread);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Mutex {
        pub context: *mut Context,
        pub platform: PlatformMutex,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union PlatformMutex {
        #[cfg(feature = "ma-win32")]
        pub win32: Win32Mutex,
        #[cfg(feature = "ma-posix")]
        pub posix: PosixMutex,
        pub unused: c_int,
    }
    impl_void_debug!(PlatformMutex);

    #[cfg(feature = "ma-win32")]
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Win32Mutex {
        pub mutex_handle: Handle,
    }
    #[cfg(feature = "ma-win32")]
    impl_void_debug!(Win32Mutex);

    #[cfg(feature = "ma-posix")]
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct PosixMutex {
        pub mutex: libc::pthread_mutex_t,
    }
    #[cfg(feature = "ma-posix")]
    impl_void_debug!(PosixMutex);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Event {
        pub context: *mut Context,
        pub platform: PlatformEvent,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union PlatformEvent {
        #[cfg(feature = "ma-win32")]
        pub win32: Win32Event,
        #[cfg(feature = "ma-posix")]
        pub posix: PosixEvent,
        pub unused: c_int,
    }
    impl_void_debug!(PlatformEvent);

    #[repr(C)]
    #[derive(Clone, Copy)]
    #[cfg(feature = "ma-win32")]
    pub struct Win32Event {
        pub event_handle: Handle,
    }
    #[cfg(feature = "ma-win32")]
    impl_void_debug!(Win32Event);

    #[repr(C)]
    #[derive(Clone, Copy)]
    #[cfg(feature = "ma-posix")]
    pub struct PosixEvent {
        pub mutex: libc::pthread_mutex_t,
        pub condition: libc::pthread_cond_t,
        pub value: u32,
    }
    #[cfg(feature = "ma-posix")]
    impl_void_debug!(PosixEvent);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Semaphore {
        pub context: *mut Context,
        pub platform: PlatformSemaphore,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union PlatformSemaphore {
        #[cfg(feature = "ma-win32")]
        pub win32: Win32Sempahore,
        #[cfg(feature = "ma-posix")]
        pub posix: PosixSemaphore,
        pub unused: c_int,
    }
    impl_void_debug!(PlatformSemaphore);

    #[repr(C)]
    #[derive(Clone, Copy)]
    #[cfg(feature = "ma-win32")]
    pub struct Win32Sempahore {
        pub semaphore_handle: Handle,
    }
    #[cfg(feature = "ma-win32")]
    impl_void_debug!(Win32Sempahore);

    #[repr(C)]
    #[derive(Clone, Copy)]
    #[cfg(feature = "ma-posix")]
    pub struct PosixSemaphore {
        pub semaphore: libc::sem_t,
    }
    #[cfg(feature = "ma-posix")]
    impl_void_debug!(PosixSemaphore);

    /// The callback for processing audio data from the device.
    ///
    /// pOutput is a pointer to a buffer that will receive audio data that will later be played back through the speakers. This will be non-null
    /// for a playback or full-duplex device and null for a capture device.
    ///
    /// pInput is a pointer to a buffer containing input data from the device. This will be non-null for a capture or full-duplex device, and
    /// null for a playback device.
    ///
    /// frameCount is the number of PCM frames to process. If an output buffer is provided (pOutput is not null), applications should write out
    /// to the entire output buffer. Note that frameCount will not necessarily be exactly what you asked for when you initialized the deviced.
    /// The bufferSizeInFrames and bufferSizeInMilliseconds members of the device config are just hints, and are not necessarily exactly what
    /// you'll get.
    ///
    /// Do _not_ call any miniaudio APIs from the callback. Attempting the stop the device can result in a deadlock. The proper way to stop the
    /// device is to call ma_device_stop() from a different thread, normally the main application thread.
    pub type DeviceCallbackProc = extern "C" fn(
        device: *mut Device,
        output: *mut c_void,
        input: *const c_void,
        frame_count: u32,
    );

    /// The callback for when the device has been stopped.
    ///
    /// This will be called when the device is stopped explicitly with ma_device_stop() and also called implicitly when the device is stopped
    /// through external forces such as being unplugged or an internal error occuring.
    ///
    /// Do not restart the device from the callback.
    pub type StopProc = extern "C" fn(device: *mut Device);

    /// The callback for handling log messages.
    ///
    /// It is possible for pDevice to be null in which case the log originated from the context. If it is non-null you can assume the message
    /// came from the device.
    ///
    /// logLevel is one of the following:
    /// - MA_LOG_LEVEL_VERBOSE
    /// - MA_LOG_LEVEL_INFO
    /// - MA_LOG_LEVEL_WARNING
    /// - MA_LOG_LEVEL_ERROR
    pub type LogProc = extern "C" fn(
        context: *mut Context,
        device: *mut Device,
        log_level: LogLevel,
        message: *const c_char,
    );

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum DeviceType {
        Playback = 1,
        Capture = 2,
        Duplex = 3,
        Loopback = 4,
    }
    impl_default!(DeviceType, DeviceType::Playback);

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum ShareMode {
        Shared = 0,
        Exclusive = 1,
    }
    impl_default!(ShareMode, ShareMode::Shared);

    /// iOS/tvOS/watchOS session categories
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum IOSSessionCategory {
        /// AVAudioSessionCategoryPlayAndRecord with AVAudioSessionCategoryOptionDefaultToSpeaker.
        Default = 0,
        /// Leave the session category unchanged.
        None = 1,
        /// AVAudioSessionCategoryAmbient
        Ambient = 2,
        /// AVAudioSessionCategorySoloAmbient
        SoloAmbient = 3,
        /// AVAudioSessionCategoryPlayback
        Playback = 4,
        /// AVAudioSessionCategoryRecord
        Record = 5,
        /// AVAudioSessionCategoryPlayAndRecord
        PlayAndRecord = 6,
        /// AVAudioSessionCategoryMultiRoute
        MultiRoute = 7,
    }

    // /// iOS/tvOS/watchOS session category options
    // #[repr(i32)]
    // #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    // pub enum IOSSessionCategoryOption {}

    bitflags::bitflags! {
        #[repr(transparent)]
        pub struct IOSSessionCategoryOption: i32 {
            /// AVAudioSessionCategoryOptionMixWithOthers
            const MIX_WITH_OTHERS = 0x01;
            /// AVAudioSessionCategoryOptionDuckOthers
            const DUCK_OTHERS = 0x02;
            /// AVAudioSessionCategoryOptionAllowBluetooth
            const ALLOW_BLUETOOTH = 0x04;
            /// AVAudioSessionCategoryOptionDefaultToSpeaker
            const DEFAULT_TO_SPEAKER = 0x08;
            /// AVAudioSessionCategoryOptionInterruptSpokenAudioAndMixWithOthers
            const INTERRUPT_SPOKEN_AUDIO_AND_MIX_WITH_OTHERS = 0x11;
            /// AVAudioSessionCategoryOptionAllowBluetoothA2DP
            const ALLOW_BLUETOOTH_A2DP = 0x20;
            /// AVAudioSessionCategoryOptionAllowAirPlay
            const ALLOW_AIR_PLAY = 0x40;
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DeviceID {
        /// WASAPI uses a wchar_t string for identification.
        #[cfg(feature = "ma-support-wasapi")]
        pub wasapi: [libc::wchar_t; 64],
        /// DirectSound uses a GUID for identification.
        #[cfg(feature = "ma-support-dsound")]
        pub dsound: [u8; 16],
        /// When creating a device, WinMM expects a Win32 UINT_PTR for device identification. In
        /// practice it's actually just a UINT.
        #[cfg(feature = "ma-support-winmm")]
        pub winmm: u32,
        /// ALSA uses a name string for identification.
        #[cfg(feature = "ma-support-alsa")]
        pub alsa: [libc::c_char; 256],
        /// PulseAudio uses a name string for identification.
        #[cfg(feature = "ma-support-pulseaudio")]
        pub pulse: [libc::c_char; 256],
        /// JACK always uses default devices.
        #[cfg(feature = "ma-support-jack")]
        pub jack: libc::c_int,
        /// Core Audio uses a string for identification.
        #[cfg(feature = "ma-support-coreaudio")]
        pub coreaudio: [libc::c_char; 256],
        /// SND/0, ect.
        #[cfg(feature = "ma-support-sndio")]
        pub sndio: [libc::c_char; 256],
        /// "/dev/audio", ect.
        #[cfg(feature = "ma-support-audio4")]
        pub audio4: [libc::c_char; 256],
        /// "dev/dsp0", ect. "dev/dsp" for default device.
        #[cfg(feature = "ma-support-oss")]
        pub oss: [libc::c_char; 64],
        /// AAudio uses a 32-bit integer for identification.
        #[cfg(feature = "ma-support-aaudio")]
        pub aaudio: i32,
        /// OpenSL|ES uses a 32-bit unsigned integer for identification.
        #[cfg(feature = "ma-support-opensl")]
        pub opensl: u32,
        /// Web Audio always uses default devices for now, but if this changes it'll be a GUID.
        #[cfg(feature = "ma-support-webaudio")]
        pub webaudio: [libc::c_char; 32],
        /// The null backend uses an integer for device IDs.
        #[cfg(feature = "ma-support-null")]
        pub nullbackend: libc::c_int,
    }
    impl_void_debug!(DeviceID);

    /// `name` and `id` are the only pieces of information guaranteed to be filled in during device
    /// enumeration.
    ///
    /// - `format_count`
    /// - `formats`
    /// - `min_channels`
    /// - `max_channels`
    /// - `min_sample_rate`
    /// - `max_sample_rate`
    ///
    /// These other details are filled in when possible using `ma_context_get_device_info()`.
    /// Note that you are allowed to initialize a device with settings outside of this range, but
    /// it just means the data will be converted using miniaudio's data conversion pipeline before
    /// sending the data to/from the device. Most programs will not need to worry about these
    /// values but it's provided here mainly for the purpose of information or in the rare case
    /// that someone might find it useful.
    ///
    /// They will be set to 0 when returned by `ma_context_enumerate_devices()` or
    /// `ma_context_get_devices()`.
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct DeviceInfo {
        pub id: DeviceID,
        pub name: [libc::c_char; 256],

        pub format_count: u32,
        pub formats: [Format; Format::COUNT],
        pub min_channels: u32,
        pub max_channels: u32,
        pub min_sample_rate: u32,
        pub max_sample_rate: u32,
    }

    impl std::fmt::Debug for DeviceInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DeviceInfo")
                .field("id", &self.id)
                .field("name", &crate::util::cstr_display(&self.name))
                .field("format_count", &self.format_count)
                .field("formats", &self.formats)
                .field("min_channels", &self.min_channels)
                .field("max_channels", &self.max_channels)
                .field("min_sample_rate", &self.min_sample_rate)
                .field("max_sample_rate", &self.max_sample_rate)
                .finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union Timer {
        pub counter: i64,
        pub counter_d: libc::c_double,
    }
    impl_void_debug!(Timer);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceConfig {
        pub device_type: DeviceType,
        pub sample_rate: u32,
        pub buffer_size_in_frames: u32,
        pub buffer_size_in_milliseconds: u32,
        pub periods: u32,
        pub performance_profile: PerformanceProfile,
        /// When set to true, the contents of the output buffer passed into the data callback will
        /// be left undefined rather than initialized to zero.
        pub no_pre_zeroed_output_buffer: Bool,
        /// When set to true, the contents of the output buffer passed into the data callback will
        /// be clipped after returning. Only applies when the playback sample format is f32.
        pub no_clip: Bool,
        pub data_callback: Option<DeviceCallbackProc>,
        pub stop_callback: Option<StopProc>,
        pub user_data: *mut c_void,
        pub playback: DeviceConfigPlayback,
        pub capture: DeviceConfigCapture,
        pub wasapi: DeviceConfigWASAPI,
        pub alsa: DeviceConfigALSA,
        pub pulse: DeviceConfigPulse,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceConfigPlayback {
        pub device_id: *mut DeviceID,
        pub format: Format,
        pub channels: u32,
        pub channel_map: [Channel; MA_MAX_CHANNELS],
        pub share_mode: ShareMode,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceConfigCapture {
        pub device_id: *mut DeviceID,
        pub format: Format,
        pub channels: u32,
        pub channel_map: [Channel; MA_MAX_CHANNELS],
        pub share_mode: ShareMode,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceConfigWASAPI {
        /// When set to true, disables the use of AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM.
        pub no_auto_convert_src: Bool,
        /// When set to true, disables the use of AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY.
        pub no_default_quality_src: Bool,
        /// Disables automatic stream routing.
        pub no_auto_stream_routing: Bool,
        /// Disables WASAPI's hardware offloading feature.
        pub no_hardware_offloading: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceConfigALSA {
        /// Disables MMap mode.
        pub no_mmap: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceConfigPulse {
        stream_name_playback: *const libc::c_char,
        stream_name_capture: *const libc::c_char,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ContextConfig {
        pub log_callback: Option<LogProc>,
        pub thread_priority: ThreadPriority,
        pub user_data: *mut c_void,
        pub alsa: ContextConfigALSA,
        pub pulse: ContextConfigPulse,
        pub coreaudio: ContextConfigCoreAudio,
        pub jack: ContextConfigJack,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ContextConfigALSA {
        pub user_verbose_device_enumeration: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ContextConfigPulse {
        pub application_name: *const libc::c_char,
        pub server_name: *const libc::c_char,
        /// Enables autospawning of the pulse audio daemon if necessary.
        pub try_auto_spawn: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ContextConfigCoreAudio {
        pub session_category: IOSSessionCategory,
        pub session_category_options: IOSSessionCategoryOption,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ContextConfigJack {
        pub client_name: *const libc::c_char,
        pub try_start_server: Bool,
    }

    pub type EnumDevicesCallbackProc = extern "C" fn(
        context: &mut Context,
        device_type: DeviceType,
        info: &DeviceInfo,
        user_data: *mut c_void,
    ) -> Bool;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Context {
        /// DirectSound, ALSA, ect.
        pub backend: Backend,
        pub log_callback: LogProc,
        pub thread_priority: ThreadPriority,
        pub user_data: *mut c_void,
        /// Used to make `ma_context_get_devices()` thread safe.
        pub device_enum_lock: Mutex,
        /// Used to make `ma_context_get_device_info()` thread safe.
        pub device_info_lock: Mutex,
        /// Total capacity of `device_infos`.
        pub device_info_capacity: u32,
        pub playback_device_info_count: u32,
        pub capture_device_info_count: u32,
        /// Playback devices first, then capture.
        pub device_infos: *mut DeviceInfo,
        bitfields: u32,
        pub on_uninit: Option<extern "C" fn(context: *mut Context) -> Result>,
        pub on_device_id_equal: Option<
            extern "C" fn(
                context: *mut Context,
                id0: *const DeviceID,
                id1: *const DeviceID,
            ) -> Bool,
        >,
        pub on_enum_devices: Option<
            extern "C" fn(
                context: *mut Context,
                callback: EnumDevicesCallbackProc,
                user_data: *mut c_void,
            ) -> Result,
        >,
        pub on_get_device_info: Option<
            extern "C" fn(
                context: *mut Context,
                device_type: DeviceType,
                device_id: DeviceID,
                share_mode: ShareMode,
                device_info: *mut DeviceInfo,
            ) -> Result,
        >,
        pub on_device_init: Option<
            extern "C" fn(
                context: *mut Context,
                config: *const DeviceConfig,
                device: *mut Device,
            ) -> Result,
        >,
        pub on_device_uninit: Option<extern "C" fn(device: *mut Device)>,
        pub on_device_start: Option<extern "C" fn(device: *mut Device) -> Result>,
        pub on_device_stop: Option<extern "C" fn(device: *mut Device) -> Result>,
        pub on_device_main_loop: Option<extern "C" fn(device: *mut Device) -> Result>,

        pub api: ContextAPI,
        pub plt: ContextPLT,
    }

    impl_bitfield!(
        Context,
        bitfields,
        set_is_backend_asynchronous,
        is_backend_asynchronous,
        1 << 0
    );

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union ContextAPI {
        #[cfg(feature = "ma-support-wasapi")]
        pub wasapi: ContextWASAPI,
        #[cfg(feature = "ma-support-dsound")]
        pub dsound: ContextDSound,
        #[cfg(feature = "ma-support-winmm")]
        pub winmm: ContextWinMM,
        #[cfg(feature = "ma-support-alsa")]
        pub alsa: ContextAlsa,
        #[cfg(feature = "ma-support-pulseaudio")]
        pub pulseaudio: ContextPulseAudio,
        #[cfg(feature = "ma-support-jack")]
        pub jack: ContextJack,
        #[cfg(feature = "ma-support-coreaudio")]
        pub coreaudio: ContextCoreAudio,
        #[cfg(feature = "ma-support-sndio")]
        pub sndio: ContextSNDIO,
        #[cfg(feature = "ma-support-oss")]
        pub oss: ContextOSS,
        #[cfg(feature = "ma-support-audio4")]
        pub audio4: ContextAudio4,
        #[cfg(feature = "ma-support-aaudio")]
        pub aaudio: ContextAAudio,
        #[cfg(feature = "ma-support-opensl")]
        pub opensl: ContextOpenSL,
        #[cfg(feature = "ma-support-webaudio")]
        pub webaudio: ContextWebAudio,
        #[cfg(feature = "ma-support-null")]
        pub null_backend: ContextNull,
    }
    impl_void_debug!(ContextAPI);

    /// Platform stuff.
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union ContextPLT {
        #[cfg(feature = "ma-win32")]
        pub win32: ContextWin32,
        #[cfg(feature = "ma-posix")]
        pub posix: ContextPosix,
        pub unused: libc::c_int,
    }
    impl_void_debug!(ContextPLT);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-wasapi")]
    pub struct ContextWASAPI {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-dsound")]
    pub struct ContextDSound {
        pub sound_dll: Handle,
        pub direct_sound_create: Proc,
        pub direct_sound_enumerate_a: Proc,
        pub direct_sound_capture_create: Proc,
        pub direct_sound_capture_enumerate_a: Proc,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-winmm")]
    pub struct ContextWinMM {
        pub win_mm: Handle,
        pub wave_out_get_num_devs: Proc,
        pub wave_out_get_dev_caps_a: Proc,
        pub wave_out_open: Proc,
        pub wave_out_close: Proc,
        pub wave_out_prepare_header: Proc,
        pub wave_out_unprepare_header: Proc,
        pub wave_out_write: Proc,
        pub wave_out_reset: Proc,
        pub wave_in_get_num_devs: Proc,
        pub wave_in_get_dev_caps_a: Proc,
        pub wave_in_open: Proc,
        pub wave_in_close: Proc,
        pub wave_in_prepare_header: Proc,
        pub wave_in_unprepare_header: Proc,
        pub wave_in_add_buffer: Proc,
        pub wave_in_start: Proc,
        pub wave_in_reset: Proc,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-alsa")]
    pub struct ContextAlsa {
        pub asound_so: Handle,
        pub snd_pcm_open: Proc,
        pub snd_pcm_close: Proc,
        pub snd_pcm_hw_params_sizeof: Proc,
        pub snd_pcm_hw_params_any: Proc,
        pub snd_pcm_hw_params_set_format: Proc,
        pub snd_pcm_hw_params_set_format_first: Proc,
        pub snd_pcm_hw_params_get_format_mask: Proc,
        pub snd_pcm_hw_params_set_channels_near: Proc,
        pub snd_pcm_hw_params_set_rate_resample: Proc,
        pub snd_pcm_hw_params_set_rate_near: Proc,
        pub snd_pcm_hw_params_set_buffer_size_near: Proc,
        pub snd_pcm_hw_params_set_periods_near: Proc,
        pub snd_pcm_hw_params_set_access: Proc,
        pub snd_pcm_hw_params_get_format: Proc,
        pub snd_pcm_hw_params_get_channels: Proc,
        pub snd_pcm_hw_params_get_channels_min: Proc,
        pub snd_pcm_hw_params_get_channels_max: Proc,
        pub snd_pcm_hw_params_get_rate: Proc,
        pub snd_pcm_hw_params_get_rate_min: Proc,
        pub snd_pcm_hw_params_get_rate_max: Proc,
        pub snd_pcm_hw_params_get_buffer_size: Proc,
        pub snd_pcm_hw_params_get_periods: Proc,
        pub snd_pcm_hw_params_get_access: Proc,
        pub snd_pcm_hw_params: Proc,
        pub snd_pcm_sw_params_sizeof: Proc,
        pub snd_pcm_sw_params_current: Proc,
        pub snd_pcm_sw_params_get_boundary: Proc,
        pub snd_pcm_sw_params_set_avail_min: Proc,
        pub snd_pcm_sw_params_set_start_threshold: Proc,
        pub snd_pcm_sw_params_set_stop_threshold: Proc,
        pub snd_pcm_sw_params: Proc,
        pub snd_pcm_format_mask_sizeof: Proc,
        pub snd_pcm_format_mask_test: Proc,
        pub snd_pcm_get_chmap: Proc,
        pub snd_pcm_state: Proc,
        pub snd_pcm_prepare: Proc,
        pub snd_pcm_start: Proc,
        pub snd_pcm_drop: Proc,
        pub snd_pcm_drain: Proc,
        pub snd_device_name_hint: Proc,
        pub snd_device_name_get_hint: Proc,
        pub snd_card_get_index: Proc,
        pub snd_device_name_free_hint: Proc,
        pub snd_pcm_mmap_begin: Proc,
        pub snd_pcm_mmap_commit: Proc,
        pub snd_pcm_recover: Proc,
        pub snd_pcm_readi: Proc,
        pub snd_pcm_writei: Proc,
        pub snd_pcm_avail: Proc,
        pub snd_pcm_avail_update: Proc,
        pub snd_pcm_wait: Proc,
        pub snd_pcm_info: Proc,
        pub snd_pcm_info_sizeof: Proc,
        pub snd_pcm_info_get_name: Proc,
        pub snd_config_update_free_global: Proc,

        pub internal_device_enum_lock: Mutex,
        pub use_verbose_device_enumeration: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-pulseaudio")]
    pub struct ContextPulseAudio {
        pub pulse_so: Handle,

        pub pa_mainloop_new: Proc,
        pub pa_mainloop_free: Proc,
        pub pa_mainloop_get_api: Proc,
        pub pa_mainloop_iterate: Proc,
        pub pa_mainloop_wakeup: Proc,
        pub pa_context_new: Proc,
        pub pa_context_unref: Proc,
        pub pa_context_connect: Proc,
        pub pa_context_disconnect: Proc,
        pub pa_context_set_state_callback: Proc,
        pub pa_context_get_state: Proc,
        pub pa_context_get_sink_info_list: Proc,
        pub pa_context_get_source_info_list: Proc,
        pub pa_context_get_sink_info_by_name: Proc,
        pub pa_context_get_source_info_by_name: Proc,
        pub pa_operation_unref: Proc,
        pub pa_operation_get_state: Proc,
        pub pa_channel_map_init_extend: Proc,
        pub pa_channel_map_valid: Proc,
        pub pa_channel_map_compatible: Proc,
        pub pa_stream_new: Proc,
        pub pa_stream_unref: Proc,
        pub pa_stream_connect_playback: Proc,
        pub pa_stream_connect_record: Proc,
        pub pa_stream_disconnect: Proc,
        pub pa_stream_get_state: Proc,
        pub pa_stream_get_sample_spec: Proc,
        pub pa_stream_get_channel_map: Proc,
        pub pa_stream_get_buffer_attr: Proc,
        pub pa_stream_set_buffer_attr: Proc,
        pub pa_stream_get_device_name: Proc,
        pub pa_stream_set_write_callback: Proc,
        pub pa_stream_set_read_callback: Proc,
        pub pa_stream_flush: Proc,
        pub pa_stream_drain: Proc,
        pub pa_stream_is_corked: Proc,
        pub pa_stream_cork: Proc,
        pub pa_stream_trigger: Proc,
        pub pa_stream_begin_write: Proc,
        pub pa_stream_write: Proc,
        pub pa_stream_peek: Proc,
        pub pa_stream_drop: Proc,
        pub pa_stream_writable_size: Proc,
        pub pa_stream_readable_size: Proc,

        pub application_name: *mut libc::c_char,
        pub server_name: *mut libc::c_char,
        pub try_auto_spawn: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-jack")]
    pub struct ContextJack {
        pub jack_so: Handle,

        pub jack_client_open: Proc,
        pub jack_client_close: Proc,
        pub jack_client_name_size: Proc,
        pub jack_set_process_callback: Proc,
        pub jack_set_buffer_size_callback: Proc,
        pub jack_on_shutdown: Proc,
        pub jack_get_sample_rate: Proc,
        pub jack_get_buffer_size: Proc,
        pub jack_get_ports: Proc,
        pub jack_activate: Proc,
        pub jack_deactivate: Proc,
        pub jack_connect: Proc,
        pub jack_port_register: Proc,
        pub jack_port_name: Proc,
        pub jack_port_get_buffer: Proc,
        pub jack_free: Proc,

        pub client_name: *mut libc::c_char,
        pub try_start_server: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-coreaudio")]
    pub struct ContextCoreAudio {
        pub core_foundation: Handle,
        pub cf_string_get_cstring: Proc,
        pub cf_release: Proc,

        pub core_audio: Handle,
        pub audio_object_get_property_data: Proc,
        pub audio_object_get_property_data_size: Proc,
        pub audio_object_set_property_data: Proc,
        pub audio_object_add_property_listener: Proc,
        pub audio_object_remove_property_listener: Proc,

        /// Could possibly be set to AudioToolbox on later version os macOS.
        pub audio_init: Handle,
        pub audio_component_find_next: Proc,
        pub audio_component_instance_dispose: Proc,
        pub audio_component_instance_new: Proc,
        pub audio_output_unit_start: Proc,
        pub audio_output_unit_stop: Proc,
        pub audio_unit_add_property_listener: Proc,
        pub audio_unit_get_property_info: Proc,
        pub audio_unit_get_property: Proc,
        pub audio_unit_set_property: Proc,
        pub audio_unit_initialize: Proc,
        pub audio_unit_render: Proc,

        /// AudioComponent
        pub component: Ptr,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-sndio")]
    pub struct ContextSNDIO {
        pub sndio_so: Handle,
        pub sio_open: Proc,
        pub sio_close: Proc,
        pub sio_setpar: Proc,
        pub sio_getpar: Proc,
        pub sio_getcap: Proc,
        pub sio_start: Proc,
        pub sio_stop: Proc,
        pub sio_read: Proc,
        pub sio_write: Proc,
        pub sio_onmove: Proc,
        pub sio_nfds: Proc,
        pub sio_pollfd: Proc,
        pub sio_revents: Proc,
        pub sio_eof: Proc,
        pub sio_setvol: Proc,
        pub sio_onvol: Proc,
        pub sio_initpar: Proc,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-audio4")]
    pub struct ContextAudio4 {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-oss")]
    pub struct ContextOSS {
        pub version_major: libc::c_int,
        pub version_minor: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-aaudio")]
    pub struct ContextAAudio {
        /// libaudio.so
        pub aaudio: Handle,
        pub aaudio_create_stream_builder: Proc,
        pub aaudio_stream_builder_delete: Proc,
        pub aaudio_stream_builder_set_device_id: Proc,
        pub aaudio_stream_builder_set_direction: Proc,
        pub aaudio_stream_builder_set_sharing_mode: Proc,
        pub aaudio_stream_builder_set_format: Proc,
        pub aaudio_stream_builder_set_channel_count: Proc,
        pub aaudio_stream_builder_set_sample_rate: Proc,
        pub aaudio_stream_builder_set_buffer_capacity_in_frames: Proc,
        pub aaudio_stream_builder_set_frames_per_data_callback: Proc,
        pub aaudio_stream_builder_set_data_callback: Proc,
        pub aaudio_stream_builder_set_error_callback: Proc,
        pub aaudio_stream_builder_set_performance_mode: Proc,
        pub aaudio_stream_builder_open_stream: Proc,
        pub aaudio_stream_close: Proc,
        pub aaudio_stream_get_state: Proc,
        pub aaudio_stream_wait_for_state_change: Proc,
        pub aaudio_stream_get_format: Proc,
        pub aaudio_stream_get_channel_count: Proc,
        pub aaudio_stream_get_sample_rate: Proc,
        pub aaudio_stream_get_buffer_capacity_in_frames: Proc,
        pub aaudio_stream_get_frames_per_data_callback: Proc,
        pub aaudio_stream_get_frames_per_burst: Proc,
        pub aaudio_stream_request_start: Proc,
        pub aaudio_stream_request_stop: Proc,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-opensl")]
    pub struct ContextOpenSL {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-webaudio")]
    pub struct ContextWebAudio {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-null")]
    pub struct ContextNull {
        pub unused: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-win32")]
    pub struct ContextWin32 {
        /// HMODULE
        pub ole32_dll: Handle,
        pub co_initialize_ex: Proc,
        pub co_uninitialize: Proc,
        pub co_create_instance: Proc,
        pub co_task_mem_free: Proc,
        pub prop_variant_clear: Proc,
        pub string_from_guid2: Proc,

        /// HMODULE
        pub user32_dll: Handle,
        pub get_foreground_window: Proc,
        pub get_desktop_window: Proc,

        /// HMODULE
        pub advapi32_dll: Handle,
        pub reg_open_key_ex_a: Proc,
        pub reg_close_key: Proc,
        pub reg_query_value_ex_a: Proc,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-posix")]
    pub struct ContextPosix {
        pub thread_so: Handle,
        pub pthread_create: Proc,
        pub pthread_join: Proc,
        pub pthread_mutex_init: Proc,
        pub pthread_mutex_destroy: Proc,
        pub pthread_mutex_lock: Proc,
        pub pthread_mutex_unlock: Proc,
        pub pthread_cond_init: Proc,
        pub pthread_cond_destroy: Proc,
        pub pthread_cond_wait: Proc,
        pub pthread_cond_signal: Proc,
        pub pthread_attr_init: Proc,
        pub pthread_attr_destroy: Proc,
        pub pthread_attr_setschedpolicy: Proc,
        pub pthread_attr_getschedparam: Proc,
        pub pthread_attr_setschedparam: Proc,
    }

    #[repr(C)]
    #[repr(align(64))]
    #[derive(Debug, Clone, Copy)]
    pub struct Device {
        pub context: *mut Context,
        pub device_type: DeviceType,
        pub sample_rate: u32,
        pub state: u32,
        pub on_data: Option<DeviceCallbackProc>,
        pub on_stop: Option<StopProc>,
        /// Application defined data.
        pub user_data: *mut libc::c_void,
        pub lock: Mutex,
        pub wakeup_event: Event,
        pub start_event: Event,
        pub stop_event: Event,
        pub thread: Thread,
        /// This is set by the worker thread after it's finished doing a job.
        pub work_result: Result,
        bitfields: u32,
        pub master_volume_factor: libc::c_float,

        pub playback: DevicePlayback,
        pub capture: DeviceCapture,
        pub api: DeviceAPI,
    }

    impl_bitfield!(
        Device,
        bitfields,
        set_using_default_sample_rate,
        using_default_sample_rate,
        1 << 0
    );

    impl_bitfield!(
        Device,
        bitfields,
        set_using_default_buffer_size,
        using_default_buffer_size,
        1 << 1
    );

    impl_bitfield!(
        Device,
        bitfields,
        set_using_default_periods,
        using_default_periods,
        1 << 2
    );

    impl_bitfield!(
        Device,
        bitfields,
        set_is_owner_of_context,
        is_owner_of_context,
        1 << 3,
        "When set to true, uninitializing, the device will also uninitialize the context. Set to true when NULL is passed into `ma_device_init()`."
    );

    impl_bitfield!(
        Device,
        bitfields,
        set_no_pre_zeroed_output_buffer,
        no_pre_zeroed_output_buffer,
        1 << 4
    );

    impl_bitfield!(Device, bitfields, set_no_clip, no_clip, 1 << 5);

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union DeviceAPI {
        #[cfg(feature = "ma-support-wasapi")]
        pub wasapi: DeviceWASAPI,
        #[cfg(feature = "ma-support-dsound")]
        pub dsound: DeviceDSound,
        #[cfg(feature = "ma-support-winmm")]
        pub winmm: DeviceWinMM,
        #[cfg(feature = "ma-support-alsa")]
        pub alsa: DeviceAlsa,
        #[cfg(feature = "ma-support-pulseaudio")]
        pub pulseaudio: DevicePulseAudio,
        #[cfg(feature = "ma-support-jack")]
        pub jack: DeviceJack,
        #[cfg(feature = "ma-support-coreaudio")]
        pub coreaudio: DeviceCoreAudio,
        #[cfg(feature = "ma-support-sndio")]
        pub sndio: DeviceSNDIO,
        #[cfg(feature = "ma-support-audio4")]
        pub audio4: DeviceAudio4,
        #[cfg(feature = "ma-support-oss")]
        pub oss: DeviceOSS,
        #[cfg(feature = "ma-support-aaudio")]
        pub aaudio: DeviceAAudio,
        #[cfg(feature = "ma-support-opensl")]
        pub opensl: DeviceOpenSL,
        #[cfg(feature = "ma-support-webaudio")]
        pub webaudio: DeviceWebAudio,
        #[cfg(feature = "ma-support-null")]
        pub null_device: DeviceNull,
    }

    impl_void_debug!(DeviceAPI);

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct DevicePlayback {
        /// Maybe temporary. Likely to be replaced with a query API.
        pub name: [libc::c_char; 256],
        /// Set to whatever was passed in when the device was initialized.
        pub share_mode: ShareMode,
        bitfields: u32,
        pub format: Format,
        pub channels: u32,
        pub channel_map: [Channel; MA_MAX_CHANNELS],
        pub internal_format: Format,
        pub internal_channels: u32,
        pub internal_sample_rate: u32,
        pub internal_channel_map: [Channel; MA_MAX_CHANNELS],
        pub internal_buffer_size_in_frames: u32,
        pub internal_periods: u32,
        pub converter: PCMConverter,
        /// Internal use only. Used as the data source when reading from the device.
        pub dsp_frame_count: u32,
        /// Internal use only. Used as the data source when reading from the device.
        pub dsp_frames: *const u8,
    }

    impl_bitfield!(
        DevicePlayback,
        bitfields,
        set_using_default_format,
        using_default_format,
        1 << 0
    );

    impl_bitfield!(
        DevicePlayback,
        bitfields,
        set_using_default_channels,
        using_default_channels,
        1 << 1
    );

    impl_bitfield!(
        DevicePlayback,
        bitfields,
        set_using_default_channel_map,
        using_default_channel_map,
        1 << 2
    );

    impl std::fmt::Debug for DevicePlayback {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DevicePlayback")
                .field("name", &crate::util::cstr_display(&self.name))
                .field("share_mode", &self.share_mode)
                .field("using_default_format", &self.using_default_format())
                .field("using_default_channels", &self.using_default_channels())
                .field(
                    "using_default_channel_map",
                    &self.using_default_channel_map(),
                )
                .field("format", &self.format)
                .field("channels", &self.channels)
                .field("channel_map", &self.channel_map)
                .field("internal_format", &self.internal_format)
                .field("internal_channels", &self.internal_channels)
                .field("internal_sample_rate", &self.internal_sample_rate)
                .field("internal_channel_map", &self.internal_channel_map)
                .field(
                    "internal_buffer_size_in_frames",
                    &self.internal_buffer_size_in_frames,
                )
                .field("internal_periods", &self.internal_periods)
                .field("converter", &self.converter)
                .field("dsp_frame_count", &self.dsp_frame_count)
                .field("dsp_frames", &self.dsp_frames)
                .finish()
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct DeviceCapture {
        /// Maybe temporary. Likely to be replaced with a query API.
        pub name: [libc::c_char; 256],
        /// Set to whatever was passed in when the device was initialized.
        pub share_mode: ShareMode,
        bitfields: u32,
        pub format: Format,
        pub channels: u32,
        pub channel_map: [Channel; MA_MAX_CHANNELS],
        pub internal_format: Format,
        pub internal_channels: u32,
        pub internal_sample_rate: u32,
        pub internal_channel_map: [Channel; MA_MAX_CHANNELS],
        pub internal_buffer_size_in_frames: u32,
        pub internal_periods: u32,
        pub converter: PCMConverter,
        /// Internal use only. Used as the data source when reading from the device.
        pub dsp_frame_count: u32,
        /// Internal use only. Used as the data source when reading from the device.
        pub dsp_frames: *const u8,
    }

    impl_bitfield!(
        DeviceCapture,
        bitfields,
        set_using_default_format,
        using_default_format,
        1 << 0
    );

    impl_bitfield!(
        DeviceCapture,
        bitfields,
        set_using_default_channels,
        using_default_channels,
        1 << 1
    );

    impl_bitfield!(
        DeviceCapture,
        bitfields,
        set_using_default_channel_map,
        using_default_channel_map,
        1 << 2
    );

    impl std::fmt::Debug for DeviceCapture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DeviceCapture")
                .field("name", &crate::util::cstr_display(&self.name))
                .field("share_mode", &self.share_mode)
                .field("using_default_format", &self.using_default_format())
                .field("using_default_channels", &self.using_default_channels())
                .field(
                    "using_default_channel_map",
                    &self.using_default_channel_map(),
                )
                .field("format", &self.format)
                .field("channels", &self.channels)
                .field("channel_map", &self.channel_map)
                .field("internal_format", &self.internal_format)
                .field("internal_channels", &self.internal_channels)
                .field("internal_sample_rate", &self.internal_sample_rate)
                .field("internal_channel_map", &self.internal_channel_map)
                .field(
                    "internal_buffer_size_in_frames",
                    &self.internal_buffer_size_in_frames,
                )
                .field("internal_periods", &self.internal_periods)
                .field("converter", &self.converter)
                .field("dsp_frame_count", &self.dsp_frame_count)
                .field("dsp_frames", &self.dsp_frames)
                .finish()
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-wasapi")]
    pub struct DeviceWASAPI {
        pub audio_client_playback: Ptr,
        pub audio_client_capture: Ptr,
        pub render_client: Ptr,
        pub capture_client: Ptr,
        pub device_enumerator: Ptr,
        pub notification_client: IMMNotificationClient,
        pub event_playback: Handle,
        pub event_capture: Handle,
        pub actual_buffer_size_in_frames_playback: u32,
        pub actual_buffer_size_in_frames_capture: u32,
        pub original_buffer_size_in_frames: u32,
        pub original_buffer_size_in_milliseconds: u32,
        pub original_periods: u32,
        pub has_default_playback_device_changed: Bool,
        pub has_default_capture_device_changed: Bool,
        pub peruiod_size_in_frames_playback: u32,
        pub period_size_in_frames_capture: u32,
        pub is_started_capture: Bool,
        pub is_started_playback: Bool,
        bitfields: u32,
    }

    #[cfg(feature = "ma-support-wasapi")]
    impl_bitfield!(
        DeviceWASAPI,
        bitfields,
        set_no_auto_convert_src,
        no_auto_convert_src,
        1 << 0
    );

    #[cfg(feature = "ma-support-wasapi")]
    impl_bitfield!(
        DeviceWASAPI,
        bitfields,
        set_no_default_quality_src,
        no_default_quality_src,
        1 << 1
    );

    #[cfg(feature = "ma-support-wasapi")]
    impl_bitfield!(
        DeviceWASAPI,
        bitfields,
        set_no_hardware_offloading,
        no_hardware_offloading,
        1 << 2
    );

    #[cfg(feature = "ma-support-wasapi")]
    impl_bitfield!(
        DeviceWASAPI,
        bitfields,
        set_allow_capture_auto_stream_routing,
        allow_capture_auto_stream_routing,
        1 << 3
    );

    #[cfg(feature = "ma-support-wasapi")]
    impl_bitfield!(
        DeviceWASAPI,
        bitfields,
        set_allow_playback_auto_stream_routing,
        allow_playback_auto_stream_routing,
        1 << 4
    );

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-dsound")]
    pub struct DeviceDSound {
        pub playback: Ptr,
        pub playback_primary_buffer: Ptr,
        pub playback_buffer: Ptr,
        pub capture: Ptr,
        pub capture_buffer: Ptr,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-winmm")]
    pub struct DeviceWinMM {
        pub device_playback: Handle,
        pub device_capture: Handle,
        pub event_playback: Handle,
        pub event_capture: Handle,

        pub fragment_size_in_frames: u32,
        pub fragment_size_in_bytes: u32,
        /// Used as index into wave_hdr_playback.
        pub next_header_playback: u32,
        /// Used as index into wave_hdr_capture.
        pub next_header_capture: u32,
        /// The number of PCM frames consumed in the buffer in wave_header.
        pub header_frames_consumed_playback: u32,
        /// The number of PCM frames consumed in the buffer in wave_header.
        pub header_grames_consumed_capture: u32,
        /// One instantiation for each period.
        pub wave_hdr_playback: *mut u8,
        /// One instantiation for each period.
        pub wave_hdr_capture: *mut u8,
        pub intermediary_buffer_playback: *mut u8,
        pub intermediary_buffer_capture: *mut u8,
        /// Used internally and is used for the heap allocated data for the intermediary buffer and
        /// the WAVEHDR structures.
        pub header_data: *mut u8,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-alsa")]
    pub struct DeviceAlsa {
        pub pcm_playback: Ptr,
        pub pcm_capture: Ptr,
        bitfields: u32,
    }

    #[cfg(feature = "ma-support-alsa")]
    impl_bitfield!(
        DeviceAlsa,
        bitfields,
        set_is_using_mmap_playback,
        is_using_mmap_playback,
        1 << 0
    );

    #[cfg(feature = "ma-support-alsa")]
    impl_bitfield!(
        DeviceAlsa,
        bitfields,
        set_is_using_mmap_capture,
        is_using_mmap_capture,
        1 << 1
    );

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-pulseaudio")]
    pub struct DevicePulseAudio {
        pub main_loop: Ptr,
        pub api: Ptr,
        pub pulse_context: Ptr,
        pub stream_playback: Ptr,
        pub stream_capture: Ptr,
        pub pulse_context_state: u32,
        pub mapped_buffer_playback: *mut libc::c_void,
        pub mapped_buffer_capture: *const libc::c_void,
        pub mapped_buffer_frames_remaining_playback: u32,
        pub mapped_buffer_frames_remaining_capture: u32,
        pub mapped_buffer_frames_capacity_playback: u32,
        pub mapped_buffer_frames_capacity_capture: u32,
        bitfields: u32,
    }

    #[cfg(feature = "ma-support-pulseaudio")]
    impl_bitfield!(
        DevicePulseAudio,
        bitfields,
        set_break_from_main_loop,
        break_from_main_loop,
        1 << 0
    );

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-jack")]
    pub struct DeviceJack {
        pub client: Ptr,
        pub ports_playback: [Ptr; MA_MAX_CHANNELS],
        pub ports_capture: [Ptr; MA_MAX_CHANNELS],
        pub intermediary_buffer_playback: *mut libc::c_float,
        pub intermediary_buffer_capture: *mut libc::c_float,
        pub duplex_rb: PCMRingBuffer,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-coreaudio")]
    pub struct DeviceCoreAudio {
        pub device_object_id_playback: u32,
        pub device_object_id_capture: u32,
        pub audio_unit_playback: Ptr,
        pub audio_unit_capture: Ptr,
        pub audio_buffer_list: Ptr,
        pub stop_event: Event,
        pub original_buffer_size_in_frames: u32,
        pub original_buffer_size_in_milliseconds: u32,
        pub original_periods: u32,
        pub is_default_playback_device: Bool,
        pub is_default_capture_device: Bool,
        /// Set to true when the default device has changed and miniaudio is in the process of
        /// switching.
        pub is_switching_playback_device: Bool,
        /// Set to true when the default device has changed and miniaudio is in the process of
        /// switching.
        pub is_switching_capture_device: Bool,
        pub duplex_rb: PCMRingBuffer,
        pub route_change_handler: *mut c_void,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-sndio")]
    pub struct DeviceSNDIO {
        pub handle_playback: Ptr,
        pub handle_capture: Ptr,
        pub is_started_playback: Bool,
        pub is_started_capture: Bool,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-audio4")]
    pub struct DeviceAudio4 {
        pub fd_playback: libc::c_int,
        pub fd_capture: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-oss")]
    pub struct DeviceOSS {
        pub fd_playback: libc::c_int,
        pub fd_capture: libc::c_int,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-aaudio")]
    pub struct DeviceAAudio {
        pub stream_playback: Ptr,
        pub stream_capture: Ptr,
        pub duplex_rb: PCMRingBuffer,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-opensl")]
    pub struct DeviceOpenSL {
        pub output_mix_obj: Ptr,
        pub output_mix: Ptr,
        pub audio_player_obj: Ptr,
        pub audio_player: Ptr,
        pub audio_recorder_obj: Ptr,
        pub audio_recorder: Ptr,
        pub buffer_queue_playback: Ptr,
        pub buffer_queue_capture: Ptr,
        pub current_buffer_index_playback: u32,
        pub current_buffer_index_capture: u32,
        /// This is malloc()'d and is used for storing audio data. Typed as u8 for easy offsetting.
        pub buffer_playback: *mut u8,
        /// This is malloc()'d and is used for storing audio data. Typed as u8 for easy offsetting.
        pub buffer_capture: *mut u8,
        pub duplex_rb: PCMRingBuffer,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-webaudio")]
    pub struct DeviceWebAudio {
        /// We use a factory on the JavaScript side to manage devices and use an index for JS/C
        /// interop.
        pub index_playback: libc::c_int,
        /// We use a factory on the JavaScript side to manage devices and use an index for JS/C
        /// interop.
        pub index_capture: libc::c_int,
        pub duplex_rb: PCMRingBuffer,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    #[cfg(feature = "ma-support-null")]
    pub struct DeviceNull {
        pub device_thread: Thread,
        pub operation_event: Event,
        pub operation_competion_event: Event,
        pub operation: u32,
        pub operation_result: Result,
        pub timer: Timer,
        pub prior_runtime: libc::c_double,
        pub current_period_frames_remaining_playback: u32,
        pub current_period_frames_remaining_capture: u32,
        pub las_processed_frame_playback: u64,
        pub last_processed_frame_capture: u32,
        pub is_started: Bool,
    }
}
