use miniaudio_sys as sys;

#[inline(always)]
pub(crate) const fn to_bool32(b: bool) -> sys::ma_bool32 {
    b as u32 as _
}

#[inline(always)]
pub(crate) const fn from_bool32(b32: sys::ma_bool32) -> bool {
    b32 != 0
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error {
    // General errors.
    Generic = sys::MA_ERROR,
    InvalidArgs = sys::MA_INVALID_ARGS,
    InvalidOperation = sys::MA_INVALID_OPERATION,
    OutOfMemory = sys::MA_OUT_OF_MEMORY,
    AccessDenied = sys::MA_ACCESS_DENIED,
    TooLarge = sys::MA_TOO_LARGE,
    Timeout = sys::MA_TIMEOUT,

    // General miniaudio-specific errors.
    FormatNotSupported = sys::MA_FORMAT_NOT_SUPPORTED,
    DeviceTypeNotSupported = sys::MA_DEVICE_TYPE_NOT_SUPPORTED,
    ShareModeNotSupported = sys::MA_SHARE_MODE_NOT_SUPPORTED,
    NoBackend = sys::MA_NO_BACKEND,
    NoDevice = sys::MA_NO_DEVICE,
    ApiNotFound = sys::MA_API_NOT_FOUND,
    InvalidDeviceConfig = sys::MA_INVALID_DEVICE_CONFIG,

    // State errors.
    DeviceBusy = sys::MA_DEVICE_BUSY,
    DeviceNotInitialized = sys::MA_DEVICE_NOT_INITIALIZED,
    DeviceNotStarted = sys::MA_DEVICE_NOT_STARTED,
    DeviceUnavailable = sys::MA_DEVICE_UNAVAILABLE,

    // Operation errors.
    FailedToMapDeviceBuffer = sys::MA_FAILED_TO_MAP_DEVICE_BUFFER,
    FailedToUnmapDeviceBuffer = sys::MA_FAILED_TO_UNMAP_DEVICE_BUFFER,
    FailedToInitBackend = sys::MA_FAILED_TO_INIT_BACKEND,
    FailedToReadDataFromClient = sys::MA_FAILED_TO_READ_DATA_FROM_CLIENT,
    FailedToReadDataFromDevice = sys::MA_FAILED_TO_READ_DATA_FROM_DEVICE,
    FailedToSendDataToClient = sys::MA_FAILED_TO_SEND_DATA_TO_CLIENT,
    FailedToSendDataToDevice = sys::MA_FAILED_TO_SEND_DATA_TO_DEVICE,
    FailedToOpenBackendDevice = sys::MA_FAILED_TO_OPEN_BACKEND_DEVICE,
    FailedToStartBackendDevice = sys::MA_FAILED_TO_START_BACKEND_DEVICE,
    FailedToStopBackendDevice = sys::MA_FAILED_TO_STOP_BACKEND_DEVICE,
    FailedToConfigureBackendDevice = sys::MA_FAILED_TO_CONFIGURE_BACKEND_DEVICE,
    FailedToCreateMutex = sys::MA_FAILED_TO_CREATE_MUTEX,
    FailedToCreateEvent = sys::MA_FAILED_TO_CREATE_EVENT,
    FailedToCreateSemaphore = sys::MA_FAILED_TO_CREATE_SEMAPHORE,
    FailedToCreateThread = sys::MA_FAILED_TO_CREATE_THREAD,
}

impl Error {
    #[inline(always)]
    pub(crate) const fn is_c_error(c_result: sys::ma_result) -> bool {
        c_result != sys::MA_SUCCESS as _
    }

    /// Converts a C result value into a `Result` with `Error`.
    pub(crate) fn from_c_result(c_result: sys::ma_result) -> Result<(), Error> {
        if !Self::is_c_error(c_result) {
            Ok(())
        } else {
            Err(Self::from_c_error(c_result))
        }
    }

    /// Converts an error from C library's int format into an `Error` enum.
    pub(crate) fn from_c_error(c_error: sys::ma_result) -> Error {
        match c_error {
            // General errors.
            sys::MA_ERROR => Error::Generic,
            sys::MA_INVALID_ARGS => Error::InvalidArgs,
            sys::MA_INVALID_OPERATION => Error::InvalidOperation,
            sys::MA_OUT_OF_MEMORY => Error::OutOfMemory,
            sys::MA_ACCESS_DENIED => Error::AccessDenied,
            sys::MA_TOO_LARGE => Error::TooLarge,
            sys::MA_TIMEOUT => Error::Timeout,

            // General miniaudio-specific errors.
            sys::MA_FORMAT_NOT_SUPPORTED => Error::FormatNotSupported,
            sys::MA_DEVICE_TYPE_NOT_SUPPORTED => Error::DeviceTypeNotSupported,
            sys::MA_SHARE_MODE_NOT_SUPPORTED => Error::ShareModeNotSupported,
            sys::MA_NO_BACKEND => Error::NoBackend,
            sys::MA_NO_DEVICE => Error::NoDevice,
            sys::MA_API_NOT_FOUND => Error::ApiNotFound,
            sys::MA_INVALID_DEVICE_CONFIG => Error::InvalidDeviceConfig,

            // State errors.
            sys::MA_DEVICE_BUSY => Error::DeviceBusy,
            sys::MA_DEVICE_NOT_INITIALIZED => Error::DeviceNotInitialized,
            sys::MA_DEVICE_NOT_STARTED => Error::DeviceNotStarted,
            sys::MA_DEVICE_UNAVAILABLE => Error::DeviceUnavailable,

            // Operation errors.
            sys::MA_FAILED_TO_MAP_DEVICE_BUFFER => Error::FailedToMapDeviceBuffer,
            sys::MA_FAILED_TO_UNMAP_DEVICE_BUFFER => Error::FailedToUnmapDeviceBuffer,
            sys::MA_FAILED_TO_INIT_BACKEND => Error::FailedToInitBackend,
            sys::MA_FAILED_TO_READ_DATA_FROM_CLIENT => Error::FailedToReadDataFromClient,
            sys::MA_FAILED_TO_READ_DATA_FROM_DEVICE => Error::FailedToReadDataFromDevice,
            sys::MA_FAILED_TO_SEND_DATA_TO_CLIENT => Error::FailedToSendDataToClient,
            sys::MA_FAILED_TO_SEND_DATA_TO_DEVICE => Error::FailedToSendDataToDevice,
            sys::MA_FAILED_TO_OPEN_BACKEND_DEVICE => Error::FailedToOpenBackendDevice,
            sys::MA_FAILED_TO_START_BACKEND_DEVICE => Error::FailedToStartBackendDevice,
            sys::MA_FAILED_TO_STOP_BACKEND_DEVICE => Error::FailedToStopBackendDevice,
            sys::MA_FAILED_TO_CONFIGURE_BACKEND_DEVICE => Error::FailedToConfigureBackendDevice,
            sys::MA_FAILED_TO_CREATE_MUTEX => Error::FailedToCreateMutex,
            sys::MA_FAILED_TO_CREATE_EVENT => Error::FailedToCreateEvent,
            sys::MA_FAILED_TO_CREATE_SEMAPHORE => Error::FailedToCreateSemaphore,
            sys::MA_FAILED_TO_CREATE_THREAD => Error::FailedToCreateThread,
            _ => Error::Generic,
        }
    }

    /// Returns the message associated with an error type.
    pub fn message(self) -> &'static str {
        match self {
            // General errors.
            Error::Generic => "generic",
            Error::InvalidArgs => "invalid args",
            Error::InvalidOperation => "invalid operation",
            Error::OutOfMemory => "out of memory",
            Error::AccessDenied => "access denied",
            Error::TooLarge => "too large",
            Error::Timeout => "timeout",

            // General miniaudio-specific errors.
            Error::FormatNotSupported => "format not supported",
            Error::DeviceTypeNotSupported => "device type not supported",
            Error::ShareModeNotSupported => "share mode not supported",
            Error::NoBackend => "no backend",
            Error::NoDevice => "no device",
            Error::ApiNotFound => "api not found",
            Error::InvalidDeviceConfig => "invalid device config",

            // State errors.
            Error::DeviceBusy => "device busy",
            Error::DeviceNotInitialized => "device not initialized",
            Error::DeviceNotStarted => "device not started",
            Error::DeviceUnavailable => "device unavailable",

            // Operation errors.
            Error::FailedToMapDeviceBuffer => "failed to map device buffer",
            Error::FailedToUnmapDeviceBuffer => "failed to unmap device buffer",
            Error::FailedToInitBackend => "failed to init backend",
            Error::FailedToReadDataFromClient => "failed to read data from client",
            Error::FailedToReadDataFromDevice => "failed to read data from device",
            Error::FailedToSendDataToClient => "failed to send data to client",
            Error::FailedToSendDataToDevice => "failed to send data to device",
            Error::FailedToOpenBackendDevice => "failed to open backend device",
            Error::FailedToStartBackendDevice => "failed to start backend device",
            Error::FailedToStopBackendDevice => "failed to stop backend device",
            Error::FailedToConfigureBackendDevice => "failed to configure backend device",
            Error::FailedToCreateMutex => "failed to create mutex",
            Error::FailedToCreateEvent => "failed to create event",
            Error::FailedToCreateSemaphore => "failed to create semaphore",
            Error::FailedToCreateThread => "failed to create thread",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl_from_c!(Channel, sys::ma_channel);

impl Channel {
    /// The default left channel.
    pub const LEFT: Self = Self::FrontLeft;
    /// The default right channel.
    pub const RIGHT: Self = Self::FrontRight;
    /// The number of channels.
    pub const COUNT: usize = sys::MA_CHANNEL_POSITION_COUNT as usize;
}

impl Default for Channel {
    fn default() -> Self {
        Self::None
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamFormat {
    PCM = sys::ma_stream_format_pcm as _,
}

impl Default for StreamFormat {
    fn default() -> Self {
        Self::PCM
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamLayout {
    Interleaved = sys::ma_stream_layout_interleaved as _,
    Deinterleaved = sys::ma_stream_layout_deinterleaved as _,
}

impl Default for StreamLayout {
    fn default() -> Self {
        Self::Interleaved
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DitherMode {
    None = sys::ma_dither_mode_none as _,
    Rectangle = sys::ma_dither_mode_rectangle as _,
    Triangle = sys::ma_dither_mode_triangle as _,
}

impl Default for DitherMode {
    fn default() -> Self {
        Self::None
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Unknown = sys::ma_format_unknown as _,
    U8 = sys::ma_format_u8 as _,
    S16 = sys::ma_format_s16 as _,
    S24 = sys::ma_format_s24 as _,
    S32 = sys::ma_format_s32 as _,
    F32 = sys::ma_format_f32 as _,
}
impl_from_c!(Format, sys::ma_format);

impl Format {
    /// Return sthe number of stream formats available.
    pub const fn count() -> usize {
        sys::ma_format_count as usize
    }

    /// The size of one sample in this format in bytes.
    pub fn size_in_bytes(self) -> usize {
        match self {
            Self::Unknown => 0,
            Self::U8 => 1,
            Self::S16 => 2,
            Self::S24 => 3,
            Self::S32 => 4,
            Self::F32 => 4,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Self::Unknown
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelMixMode {
    /// Simple averaging based on the plane(s) the channel is sitting on.
    Rectangular = sys::ma_channel_mix_mode_rectangular as _,
    /// Drop excess channels; zeroed out extra channels.
    Simple = sys::ma_channel_mix_mode_simple as _,
    /// Use custom weights specified in `ChannelRouterConfig`.
    CustomWeights = sys::ma_channel_mix_mode_custom_weights as _,
}
impl_from_c!(ChannelMixMode, sys::ma_channel_mix_mode);

impl ChannelMixMode {
    pub const PLANAR_BLEND: Self = Self::Rectangular;
    pub const DEFAULT: Self = Self::PLANAR_BLEND;
}

impl Default for ChannelMixMode {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardChannelMap {
    Microsoft = sys::ma_standard_channel_map_microsoft as _,
    Alsa = sys::ma_standard_channel_map_alsa as _,
    /// Based on AIFF.
    Rfc3551 = sys::ma_standard_channel_map_rfc3551 as _,
    Flac = sys::ma_standard_channel_map_flac as _,
    Vorbis = sys::ma_standard_channel_map_vorbis as _,
    /// FreeBSD's sound(4).
    Sound4 = sys::ma_standard_channel_map_sound4 as _,
    /// https://www.sndio.org/tips.html
    Sndio = sys::ma_standard_channel_map_sndio as _,
}
impl_from_c!(StandardChannelMap, sys::ma_standard_channel_map);

impl StandardChannelMap {
    /// https://webaudio.github.io/web-audio-api/#ChannelOrdering.
    /// Only 1, 2, 4 and 6 channels are defined, but can fill in the gaps with logical assumptions.
    pub const WEBAUDIO: Self = Self::Flac;
    pub const DEFAULT: Self = Self::Microsoft;
}

impl Default for StandardChannelMap {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceProfile {
    LowLatency = sys::ma_performance_profile_low_latency as _,
    Conservative = sys::ma_performance_profile_conservative as _,
}
impl_from_c!(PerformanceProfile, sys::ma_performance_profile);

impl Default for PerformanceProfile {
    fn default() -> Self {
        Self::LowLatency
    }
}

// Standard Sample Rates:
pub const SAMPLE_RATE_8000: u32 = sys::MA_SAMPLE_RATE_8000;
pub const SAMPLE_RATE_11025: u32 = sys::MA_SAMPLE_RATE_11025;
pub const SAMPLE_RATE_16000: u32 = sys::MA_SAMPLE_RATE_16000;
pub const SAMPLE_RATE_22050: u32 = sys::MA_SAMPLE_RATE_22050;
pub const SAMPLE_RATE_24000: u32 = sys::MA_SAMPLE_RATE_24000;
pub const SAMPLE_RATE_32000: u32 = sys::MA_SAMPLE_RATE_32000;
pub const SAMPLE_RATE_44100: u32 = sys::MA_SAMPLE_RATE_44100;
pub const SAMPLE_RATE_48000: u32 = sys::MA_SAMPLE_RATE_48000;
pub const SAMPLE_RATE_88200: u32 = sys::MA_SAMPLE_RATE_88200;
pub const SAMPLE_RATE_96000: u32 = sys::MA_SAMPLE_RATE_96000;
pub const SAMPLE_RATE_176400: u32 = sys::MA_SAMPLE_RATE_176400;
pub const SAMPLE_RATE_192000: u32 = sys::MA_SAMPLE_RATE_192000;
pub const SAMPLE_RATE_352800: u32 = sys::MA_SAMPLE_RATE_352800;
pub const SAMPLE_RATE_384000: u32 = sys::MA_SAMPLE_RATE_384000;

pub const MIN_SAMPLE_RATE: u32 = SAMPLE_RATE_8000;
pub const MAX_SAMPLE_RATE: u32 = SAMPLE_RATE_384000;

/// Minimum number of channels in a channel map.
pub const MIN_CHANNELS: usize = sys::MA_MIN_CHANNELS as usize;

/// Maximum number of channels in a channel map.
pub const MAX_CHANNELS: usize = sys::MA_MAX_CHANNELS as usize;

pub const MAX_FILTER_POLES: usize = 8;
