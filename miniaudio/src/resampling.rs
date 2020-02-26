use miniaudio_sys as sys;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResampleAlgorithm {
    Linear = sys::ma_resample_algorithm_linear as _,
    Speex = sys::ma_resample_algorithm_speex as _,
}
impl_from_c!(ResampleAlgorithm, sys::ma_resample_algorithm);
