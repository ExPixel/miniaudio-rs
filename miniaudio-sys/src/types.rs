use super::constants::*;
use std::os::raw::{c_float, c_void};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum MAResult {
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
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StreamFormat {
    PCM = 0,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StreamLayout {
    Interleaved = 0,
    Deinterleaved = 1,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum DitherMode {
    None = 0,
    Rectangle = 1,
    Triangle = 2,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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

macro_rules! impl_bitfield {
    ($ForType:ty, $BitField:ident, $Set:ident, $Get:ident, $Mask:expr) => {
        impl $ForType {
            pub fn $Set(&mut self, value: bool) {
                if value {
                    self.$BitField |= $Mask;
                } else {
                    self.$BitField &= !($Mask);
                }
            }

            pub fn $Get(&self) -> bool {
                (self.$BitField & $Mask) != 0
            }
        }
    };
}

macro_rules! impl_simd_bitfields {
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
#[derive(Clone, Copy)]
pub struct FormatConverterConfig {
    pub format_in: Format,
    pub format_out: Format,
    pub channels: u32,
    pub stream_format_in: StreamFormat,
    pub stream_format_out: StreamFormat,
    pub dither_mode: DitherMode,
    simd_bits: u32,
    pub on_read: FormatConverterReadProc,
    pub on_read_deinterleaved: FormatConverterReadDeinterleavedProc,
    pub user_data: *mut c_void,
}
impl_simd_bitfields!(FormatConverterConfig, simd_bits, 0);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FormatConverter {
    pub config: FormatConverterConfig,
    pub simd_bits: u32,
    pub on_convert_pcm:
        extern "C" fn(dst: *mut c_void, src: *const c_void, count: u64, dither_mode: DitherMode),
    pub on_interleave_pcm:
        extern "C" fn(dst: *mut c_void, src: *const *const c_void, frame_count: u64, channels: u32),
    pub on_deinterleave_pcm:
        extern "C" fn(dst: *mut *mut c_void, src: *const c_void, frame_count: u64, channels: u32),
}
impl_simd_bitfields!(FormatConverter, simd_bits, 0);

pub type ChannelRouterReadDeinterleavedProc = extern "C" fn(
    router: *mut ChannelRouter,
    frame_count: u32,
    pp_samples_out: *mut *mut c_void,
    user_data: *mut c_void,
) -> u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelRouterConfig {
    pub channels_in: u32,
    pub channels_out: u32,
    pub channel_map_in: [Channel; MA_MAX_CHANNELS as usize],
    pub channel_map_out: [Channel; MA_MAX_CHANNELS as usize],
    pub mixing_mode: ChannelMixMode,
    pub weights: [[c_float; MA_MAX_CHANNELS as usize]; MA_MAX_CHANNELS as usize],
    pub simd_bits: u32,
    pub on_read_deinterleaved: ChannelRouterReadDeinterleavedProc,
    pub user_data: *mut c_void,
}
impl_simd_bitfields!(ChannelRouterConfig, simd_bits, 0);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelRouter {
    pub config: ChannelRouterConfig,
    bitfields: u32,
    pub shuffle_table: [u8; MA_MAX_CHANNELS as usize],
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
impl_simd_bitfields!(ChannelRouter, bitfields, 4);

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Clone, Copy)]
pub struct SrcConfigSinc {
    pub window_function: SrcSincWindowFunction,
    pub window_width: u32,
}

/// Returns the number of frames that were read.
pub type SrcReadDeinterleavedProc = extern "C" fn(
    p_src: *mut Src,
    frame_count: u32,
    pp_samples_out: *mut *mut c_void,
    user_data: *mut c_void,
) -> u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SrcConfig {
    pub sample_rate_in: u32,
    pub sample_rate_out: u32,
    pub channels: u32,
    pub algorithm: SrcAlgorithm,
    bitfields: u32,
    pub on_read_deinterleaved: SrcReadDeinterleavedProc,
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
impl_simd_bitfields!(SrcConfig, bitfields, 1);

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct Src {
    pub inner: SrcInnerUnion,
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
impl_simd_bitfields!(Src, bitfields, 1);

#[repr(align(64))]
#[derive(Clone, Copy)]
pub union SrcInnerUnion {
    pub linear: SrcLinear,
    pub sinc: SrcSinc,
}

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct SrcLinear {
    input: [[c_float; MA_MAX_CHANNELS as usize]; MA_SRC_INPUT_BUFFER_SIZE_IN_SAMPLES as usize],
    time_in: c_float,
    left_over_frames: u32,
}

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct SrcSinc {
    input: [[c_float; MA_MAX_CHANNELS as usize];
        MA_SRC_SINC_MAX_WINDOW_WIDTH as usize * 2 + MA_SRC_INPUT_BUFFER_SIZE_IN_SAMPLES as usize],
    time_in: c_float,
    /// The number of frames sitting in the input buffer, not including the first half of the
    /// window.
    input_frame_count: u32,
    /// An offset of `input`.
    window_pos_in_samples: u32,
    /// Precomputed lookup table. The +1 is used to avoid the need for an overflow check.
    table: [c_float;
        MA_SRC_SINC_MAX_WINDOW_WIDTH as usize * MA_SRC_SINC_LOOKUP_TABLE_RESOLUTION as usize],
}
