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
mod biquad_filtering;
mod device_io;
mod frames;
mod generation;
mod low_pass_filtering;
mod resampling;
mod ring_buffers;

pub use base::*;
pub use biquad_filtering::*;
pub use device_io::*;
pub use frames::*;
pub use generation::*;
pub use low_pass_filtering::*;
pub use resampling::*;
pub use ring_buffers::*;
