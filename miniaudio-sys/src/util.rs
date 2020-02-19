/// If this is a valid NUL terminated C string, this function will return
/// it as a Rust str reference. If there is no NUL terminator, this will just consider
/// the entire array as the C string.
///
/// **NOTE** This function is only for debug purposes and may allocate if invalud UTF-8 sequences
/// are present in the string.
pub fn cstr_display<'s>(full_arr: &'s [libc::c_char]) -> std::borrow::Cow<'s, str> {
    let byte_slice = into_byte_slice(full_arr);

    if let Ok(cstr) = std::ffi::CStr::from_bytes_with_nul(byte_slice) {
        cstr.to_string_lossy()
    } else {
        // if there is no nul terminator, just use the entire slice:
        String::from_utf8_lossy(byte_slice)
    }
}

/// Converts slice of any sized type into a slice of bytes.
pub fn into_byte_slice<T: Sized>(orig: &[T]) -> &[u8] {
    // FIXME I don't think the behavior here is undefined since u8 should have an alignment of 1, but
    // I might be wrong :P

    let byte_len = orig.len() * std::mem::size_of::<T>();
    let ptr = orig.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, byte_len) }
}
