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

/// 32-bit boolean value used by mini-audio.
#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Bool(u32);

impl Bool {
    pub const TRUE: Bool = Bool(1);
    pub const FALSE: Bool = Bool(0);
}

impl Default for Bool {
    #[inline]
    fn default() -> Bool {
        Self::FALSE
    }
}

impl From<Bool> for bool {
    #[inline]
    fn from(b: Bool) -> bool {
        b.0 != 0
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

impl std::ops::Not for Bool {
    type Output = Bool;

    #[inline]
    fn not(self) -> Bool {
        if bool::from(self) {
            Bool::FALSE
        } else {
            Bool::TRUE
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

#[cfg(not(feature = "ma-no-device-io"))]
mod device_io {
    use super::LogLevel;
    use libc::{c_char, c_int, c_void};

    #[cfg(feature = "ma-support-wasapi")]
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

    #[cfg(feature = "ma-win32")]
    #[derive(Clone, Copy)]
    pub struct Win32Event {
        pub event_handle: Handle,
    }
    #[cfg(feature = "ma-win32")]
    impl_void_debug!(Win32Event);

    #[cfg(feature = "ma-posix")]
    #[derive(Clone, Copy)]
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

    #[cfg(feature = "ma-win32")]
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Win32Sempahore {
        pub semaphore_handle: Handle,
    }
    #[cfg(feature = "ma-win32")]
    impl_void_debug!(Win32Sempahore);

    #[cfg(feature = "ma-posix")]
    #[repr(C)]
    #[derive(Clone, Copy)]
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

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum ShareMode {
        Shared = 0,
        Exclusive = 1,
    }

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

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Context {
        x: u32,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Device {}
}
