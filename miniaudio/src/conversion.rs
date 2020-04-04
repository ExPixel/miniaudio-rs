use crate::base::Format;
use miniaudio_sys as sys;
use std::os::raw::c_void;

pub type ConvertFn =
    unsafe extern "C" fn(*mut c_void, *const c_void, count: u64, dither_mode: sys::ma_dither_mode);

pub(crate) fn conversion_fn_for(input_format: Format, output_format: Format) -> ConvertFn {
    match (input_format, output_format) {
        (Format::U8, Format::S16) => sys::ma_pcm_u8_to_s16,
        (Format::U8, Format::S24) => sys::ma_pcm_u8_to_s24,
        (Format::U8, Format::S32) => sys::ma_pcm_u8_to_s32,
        (Format::U8, Format::F32) => sys::ma_pcm_u8_to_f32,
        (Format::S16, Format::U8) => sys::ma_pcm_s16_to_u8,
        (Format::S16, Format::S24) => sys::ma_pcm_s16_to_s24,
        (Format::S16, Format::S32) => sys::ma_pcm_s16_to_s32,
        (Format::S16, Format::F32) => sys::ma_pcm_s16_to_f32,
        (Format::S24, Format::U8) => sys::ma_pcm_s24_to_u8,
        (Format::S24, Format::S16) => sys::ma_pcm_s24_to_s16,
        (Format::S24, Format::S32) => sys::ma_pcm_s24_to_s32,
        (Format::S24, Format::F32) => sys::ma_pcm_s24_to_f32,
        (Format::S32, Format::U8) => sys::ma_pcm_s32_to_u8,
        (Format::S32, Format::S16) => sys::ma_pcm_s32_to_s16,
        (Format::S32, Format::S24) => sys::ma_pcm_s32_to_s24,
        (Format::S32, Format::F32) => sys::ma_pcm_s32_to_f32,
        (Format::F32, Format::U8) => sys::ma_pcm_f32_to_u8,
        (Format::F32, Format::S16) => sys::ma_pcm_f32_to_s16,
        (Format::F32, Format::S24) => sys::ma_pcm_f32_to_s24,
        (Format::F32, Format::S32) => sys::ma_pcm_f32_to_s32,
        (Format::U8, Format::U8) => copy_converter_u8,
        (Format::S16, Format::S16) => copy_converter_s16,
        (Format::S24, Format::S24) => copy_converter_s24,
        (Format::S32, Format::S32) => copy_converter_s32,
        (Format::F32, Format::F32) => copy_converter_f32,
        (Format::Unknown, _) => bad_converter,
        (_, Format::Unknown) => bad_converter,
    }
}

unsafe fn copy_converter<T: Sized + Copy>(output: *mut T, input: *const T, count: usize) {
    let output_slice = std::slice::from_raw_parts_mut(output, count);
    let input_slice = std::slice::from_raw_parts(input, count);
    output_slice.copy_from_slice(input_slice);
}

unsafe extern "C" fn copy_converter_u8(
    output: *mut c_void,
    input: *const c_void,
    count: u64,
    _dither_mode: sys::ma_dither_mode,
) {
    copy_converter::<u8>(output as *mut _, input as *const _, count as usize);
}

unsafe extern "C" fn copy_converter_s16(
    output: *mut c_void,
    input: *const c_void,
    count: u64,
    _dither_mode: sys::ma_dither_mode,
) {
    copy_converter::<i16>(output as *mut _, input as *const _, count as usize);
}

unsafe extern "C" fn copy_converter_s24(
    output: *mut c_void,
    input: *const c_void,
    count: u64,
    _dither_mode: sys::ma_dither_mode,
) {
    copy_converter::<[u8; 3]>(output as *mut _, input as *const _, count as usize);
}

unsafe extern "C" fn copy_converter_s32(
    output: *mut c_void,
    input: *const c_void,
    count: u64,
    _dither_mode: sys::ma_dither_mode,
) {
    copy_converter::<i32>(output as *mut _, input as *const _, count as usize);
}

unsafe extern "C" fn copy_converter_f32(
    output: *mut c_void,
    input: *const c_void,
    count: u64,
    _dither_mode: sys::ma_dither_mode,
) {
    copy_converter::<f32>(output as *mut _, input as *const _, count as usize);
}

unsafe extern "C" fn bad_converter(
    _out: *mut c_void,
    _in: *const c_void,
    _count: u64,
    _dither_mode: sys::ma_dither_mode,
) {
    ma_debug_panic!("converting from or to unknown format")
}
