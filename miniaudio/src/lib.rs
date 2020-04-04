/// This is a panic which only occurs in debug mode.
#[cfg(debug_assertions)]
const MA_DEBUG_PANIC: bool = true;
#[cfg(not(debug_assertions))]
const MA_DEBUG_PANIC: bool = false;

macro_rules! ma_debug_panic {
    ($($Arg:expr,)*) => {
        if $crate::MA_DEBUG_PANIC {
            panic!($($Arg,)*)
        }
    };

    ($($Arg:expr),*) => {
        if $crate::MA_DEBUG_PANIC {
            panic!($($Arg,)*)
        }
    };
}

/// This macro will execute a success block if a result is a MA_SUCCESS block
/// and return the value of that block wrapped in a Result::Ok. If $Result is an error this will
/// return an Error enum wrapped in a Result::Err.
macro_rules! map_result {
    ($Result:expr, $Success:expr) => {
        if $crate::base::Error::is_c_error($Result) {
            Err($crate::base::Error::from_c_error($Result))
        } else {
            Ok($Success)
        }
    };
}

macro_rules! impl_from_c {
    ($RustType:ty, $CType:ty) => {
        impl $RustType {
            pub fn from_c(c_enum: $CType) -> $RustType {
                unsafe { std::mem::transmute(c_enum) }
            }
        }
    };
}

mod base;
mod channel_conv;
// mod data_conv;
mod device_io;
mod frames;

// pub mod filters;
pub mod generation;
pub mod resampling;
pub mod ring_buffers;

pub use base::*;
pub use channel_conv::*;
// pub use data_conv::*;
pub use device_io::*;
pub use frames::*;
