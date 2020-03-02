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
    Generic = sys::MA_ERROR,
    InvalidArgs = sys::MA_INVALID_ARGS,
    InvalidOperation = sys::MA_INVALID_OPERATION,
    OutOfMemory = sys::MA_OUT_OF_MEMORY,
    OutOfRange = sys::MA_OUT_OF_RANGE,
    AccessDenied = sys::MA_ACCESS_DENIED,
    DoesNotExist = sys::MA_DOES_NOT_EXIST,
    AlreadyExists = sys::MA_ALREADY_EXISTS,
    TooManyOpenFiles = sys::MA_TOO_MANY_OPEN_FILES,
    InvalidFile = sys::MA_INVALID_FILE,
    TooBig = sys::MA_TOO_BIG,
    PathTooLong = sys::MA_PATH_TOO_LONG,
    NameTooLong = sys::MA_NAME_TOO_LONG,
    NotDirectory = sys::MA_NOT_DIRECTORY,
    IsDirectory = sys::MA_IS_DIRECTORY,
    DirectoryNotEmpty = sys::MA_DIRECTORY_NOT_EMPTY,
    EndOfFile = sys::MA_END_OF_FILE,
    NoSpace = sys::MA_NO_SPACE,
    Busy = sys::MA_BUSY,
    IoError = sys::MA_IO_ERROR,
    Interrupt = sys::MA_INTERRUPT,
    Unavailable = sys::MA_UNAVAILABLE,
    AlreadyInUse = sys::MA_ALREADY_IN_USE,
    BadAddress = sys::MA_BAD_ADDRESS,
    BadSeek = sys::MA_BAD_SEEK,
    BadPipe = sys::MA_BAD_PIPE,
    Deadlock = sys::MA_DEADLOCK,
    TooManyLinks = sys::MA_TOO_MANY_LINKS,
    NotImplemented = sys::MA_NOT_IMPLEMENTED,
    NoMessage = sys::MA_NO_MESSAGE,
    BadMessage = sys::MA_BAD_MESSAGE,
    NoDataAvailable = sys::MA_NO_DATA_AVAILABLE,
    InvalidData = sys::MA_INVALID_DATA,
    Timeout = sys::MA_TIMEOUT,
    NoNetwork = sys::MA_NO_NETWORK,
    NotUnique = sys::MA_NOT_UNIQUE,
    NotSocket = sys::MA_NOT_SOCKET,
    NoAddress = sys::MA_NO_ADDRESS,
    BadProtocol = sys::MA_BAD_PROTOCOL,
    ProtocolUnavailable = sys::MA_PROTOCOL_UNAVAILABLE,
    ProtocolNotSupported = sys::MA_PROTOCOL_NOT_SUPPORTED,
    ProtocolFamilyNotSupported = sys::MA_PROTOCOL_FAMILY_NOT_SUPPORTED,
    AddressFamilyNotSupported = sys::MA_ADDRESS_FAMILY_NOT_SUPPORTED,
    SocketNotSupported = sys::MA_SOCKET_NOT_SUPPORTED,
    ConnectionReset = sys::MA_CONNECTION_RESET,
    AlreadyConnected = sys::MA_ALREADY_CONNECTED,
    NotConnected = sys::MA_NOT_CONNECTED,
    ConnectionRefused = sys::MA_CONNECTION_REFUSED,
    NoHost = sys::MA_NO_HOST,
    InProgress = sys::MA_IN_PROGRESS,
    Cancelled = sys::MA_CANCELLED,
    MemoryAlreadyMapped = sys::MA_MEMORY_ALREADY_MAPPED,
    AtEnd = sys::MA_AT_END,

    /* General miniaudio-specific errors. */
    FormatNotSupported = sys::MA_FORMAT_NOT_SUPPORTED,
    DeviceTypeNotSupported = sys::MA_DEVICE_TYPE_NOT_SUPPORTED,
    ShareModeNotSupported = sys::MA_SHARE_MODE_NOT_SUPPORTED,
    NoBackend = sys::MA_NO_BACKEND,
    NoDevice = sys::MA_NO_DEVICE,
    ApiNotFound = sys::MA_API_NOT_FOUND,
    InvalidDeviceConfig = sys::MA_INVALID_DEVICE_CONFIG,

    /* State errors. */
    DeviceNotInitialized = sys::MA_DEVICE_NOT_INITIALIZED,
    DeviceAlreadyInitialized = sys::MA_DEVICE_ALREADY_INITIALIZED,
    DeviceNotStarted = sys::MA_DEVICE_NOT_STARTED,
    DeviceNotStopped = sys::MA_DEVICE_NOT_STOPPED,

    /* Operation errors. */
    FailedToInitBackend = sys::MA_FAILED_TO_INIT_BACKEND,
    FailedToOpenBackendDevice = sys::MA_FAILED_TO_OPEN_BACKEND_DEVICE,
    FailedToStartBackendDevice = sys::MA_FAILED_TO_START_BACKEND_DEVICE,
    FailedToStopBackendDevice = sys::MA_FAILED_TO_STOP_BACKEND_DEVICE,
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
            sys::MA_INVALID_ARGS => Error::InvalidArgs,
            sys::MA_INVALID_OPERATION => Error::InvalidOperation,
            sys::MA_OUT_OF_MEMORY => Error::OutOfMemory,
            sys::MA_OUT_OF_RANGE => Error::OutOfRange,
            sys::MA_ACCESS_DENIED => Error::AccessDenied,
            sys::MA_DOES_NOT_EXIST => Error::DoesNotExist,
            sys::MA_ALREADY_EXISTS => Error::AlreadyExists,
            sys::MA_TOO_MANY_OPEN_FILES => Error::TooManyOpenFiles,
            sys::MA_INVALID_FILE => Error::InvalidFile,
            sys::MA_TOO_BIG => Error::TooBig,
            sys::MA_PATH_TOO_LONG => Error::PathTooLong,
            sys::MA_NAME_TOO_LONG => Error::NameTooLong,
            sys::MA_NOT_DIRECTORY => Error::NotDirectory,
            sys::MA_IS_DIRECTORY => Error::IsDirectory,
            sys::MA_DIRECTORY_NOT_EMPTY => Error::DirectoryNotEmpty,
            sys::MA_END_OF_FILE => Error::EndOfFile,
            sys::MA_NO_SPACE => Error::NoSpace,
            sys::MA_BUSY => Error::Busy,
            sys::MA_IO_ERROR => Error::IoError,
            sys::MA_INTERRUPT => Error::Interrupt,
            sys::MA_UNAVAILABLE => Error::Unavailable,
            sys::MA_ALREADY_IN_USE => Error::AlreadyInUse,
            sys::MA_BAD_ADDRESS => Error::BadAddress,
            sys::MA_BAD_SEEK => Error::BadSeek,
            sys::MA_BAD_PIPE => Error::BadPipe,
            sys::MA_DEADLOCK => Error::Deadlock,
            sys::MA_TOO_MANY_LINKS => Error::TooManyLinks,
            sys::MA_NOT_IMPLEMENTED => Error::NotImplemented,
            sys::MA_NO_MESSAGE => Error::NoMessage,
            sys::MA_BAD_MESSAGE => Error::BadMessage,
            sys::MA_NO_DATA_AVAILABLE => Error::NoDataAvailable,
            sys::MA_INVALID_DATA => Error::InvalidData,
            sys::MA_TIMEOUT => Error::Timeout,
            sys::MA_NO_NETWORK => Error::NoNetwork,
            sys::MA_NOT_UNIQUE => Error::NotUnique,
            sys::MA_NOT_SOCKET => Error::NotSocket,
            sys::MA_NO_ADDRESS => Error::NoAddress,
            sys::MA_BAD_PROTOCOL => Error::BadProtocol,
            sys::MA_PROTOCOL_UNAVAILABLE => Error::ProtocolUnavailable,
            sys::MA_PROTOCOL_NOT_SUPPORTED => Error::ProtocolNotSupported,
            sys::MA_PROTOCOL_FAMILY_NOT_SUPPORTED => Error::ProtocolFamilyNotSupported,
            sys::MA_ADDRESS_FAMILY_NOT_SUPPORTED => Error::AddressFamilyNotSupported,
            sys::MA_SOCKET_NOT_SUPPORTED => Error::SocketNotSupported,
            sys::MA_CONNECTION_RESET => Error::ConnectionReset,
            sys::MA_ALREADY_CONNECTED => Error::AlreadyConnected,
            sys::MA_NOT_CONNECTED => Error::NotConnected,
            sys::MA_CONNECTION_REFUSED => Error::ConnectionRefused,
            sys::MA_NO_HOST => Error::NoHost,
            sys::MA_IN_PROGRESS => Error::InProgress,
            sys::MA_CANCELLED => Error::Cancelled,
            sys::MA_MEMORY_ALREADY_MAPPED => Error::MemoryAlreadyMapped,
            sys::MA_AT_END => Error::AtEnd,

            /* General miniaudio-specific errors. */
            sys::MA_FORMAT_NOT_SUPPORTED => Error::FormatNotSupported,
            sys::MA_DEVICE_TYPE_NOT_SUPPORTED => Error::DeviceTypeNotSupported,
            sys::MA_SHARE_MODE_NOT_SUPPORTED => Error::ShareModeNotSupported,
            sys::MA_NO_BACKEND => Error::NoBackend,
            sys::MA_NO_DEVICE => Error::NoDevice,
            sys::MA_API_NOT_FOUND => Error::ApiNotFound,
            sys::MA_INVALID_DEVICE_CONFIG => Error::InvalidDeviceConfig,

            /* State errors. */
            sys::MA_DEVICE_NOT_INITIALIZED => Error::DeviceNotInitialized,
            sys::MA_DEVICE_ALREADY_INITIALIZED => Error::DeviceAlreadyInitialized,
            sys::MA_DEVICE_NOT_STARTED => Error::DeviceNotStarted,
            sys::MA_DEVICE_NOT_STOPPED => Error::DeviceNotStopped,

            /* Operation errors. */
            sys::MA_FAILED_TO_INIT_BACKEND => Error::FailedToInitBackend,
            sys::MA_FAILED_TO_OPEN_BACKEND_DEVICE => Error::FailedToOpenBackendDevice,
            sys::MA_FAILED_TO_START_BACKEND_DEVICE => Error::FailedToStartBackendDevice,
            sys::MA_FAILED_TO_STOP_BACKEND_DEVICE => Error::FailedToStopBackendDevice,
            _ => Error::Generic,
        }
    }

    /// Returns the message associated with an error type.
    pub fn message(self) -> &'static str {
        match self {
            Error::Generic => "generic",
            Error::InvalidArgs => "invalid args",
            Error::InvalidOperation => "invalid operation",
            Error::OutOfMemory => "out of memory",
            Error::OutOfRange => "out of range",
            Error::AccessDenied => "access denied",
            Error::DoesNotExist => "does not exist",
            Error::AlreadyExists => "already exists",
            Error::TooManyOpenFiles => "too many open files",
            Error::InvalidFile => "invalid file",
            Error::TooBig => "too big",
            Error::PathTooLong => "path too long",
            Error::NameTooLong => "name too long",
            Error::NotDirectory => "not directory",
            Error::IsDirectory => "is directory",
            Error::DirectoryNotEmpty => "directory not empty",
            Error::EndOfFile => "end of file",
            Error::NoSpace => "no space",
            Error::Busy => "busy",
            Error::IoError => "io error",
            Error::Interrupt => "interrupt",
            Error::Unavailable => "unavailable",
            Error::AlreadyInUse => "already in use",
            Error::BadAddress => "bad address",
            Error::BadSeek => "bad seek",
            Error::BadPipe => "bad pipe",
            Error::Deadlock => "deadlock",
            Error::TooManyLinks => "too many links",
            Error::NotImplemented => "not implemented",
            Error::NoMessage => "no message",
            Error::BadMessage => "bad message",
            Error::NoDataAvailable => "no data available",
            Error::InvalidData => "invalid data",
            Error::Timeout => "timeout",
            Error::NoNetwork => "no network",
            Error::NotUnique => "not unique",
            Error::NotSocket => "not socket",
            Error::NoAddress => "no address",
            Error::BadProtocol => "bad protocol",
            Error::ProtocolUnavailable => "protocol unavailable",
            Error::ProtocolNotSupported => "protocol not supported",
            Error::ProtocolFamilyNotSupported => "protocol family not supported",
            Error::AddressFamilyNotSupported => "address family not supported",
            Error::SocketNotSupported => "socket not supported",
            Error::ConnectionReset => "connection reset",
            Error::AlreadyConnected => "already connected",
            Error::NotConnected => "not connected",
            Error::ConnectionRefused => "connection refused",
            Error::NoHost => "no host",
            Error::InProgress => "in progress",
            Error::Cancelled => "cancelled",
            Error::MemoryAlreadyMapped => "memory already mapped",
            Error::AtEnd => "at end",

            /* General miniaudio-specific errors. */
            Error::FormatNotSupported => "format not supported",
            Error::DeviceTypeNotSupported => "device type not supported",
            Error::ShareModeNotSupported => "share mode not supported",
            Error::NoBackend => "no backend",
            Error::NoDevice => "no device",
            Error::ApiNotFound => "api not found",
            Error::InvalidDeviceConfig => "invalid device config",

            /* State errors. */
            Error::DeviceNotInitialized => "device not initialized",
            Error::DeviceAlreadyInitialized => "device already initialized",
            Error::DeviceNotStarted => "device not started",
            Error::DeviceNotStopped => "device not stopped",

            /* Operation errors. */
            Error::FailedToInitBackend => "failed to init backend",
            Error::FailedToOpenBackendDevice => "failed to open backend device",
            Error::FailedToStartBackendDevice => "failed to start backend device",
            Error::FailedToStopBackendDevice => "failed to stop backend device",
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
