use crate::constants::*;

// #[cfg(not(feature = "ma-no-device-io"))]
// pub use device_io::*;

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

#[repr(C)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StreamFormat {
    PCM = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum StreamLayout {
    Interleaved = 0,
    Deinterleaved = 1,
}

#[repr(C)]
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
