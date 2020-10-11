#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub mod util;

include!(env!("MINIAUDIO_SYS_BINDINGS_FILE"));

#[cfg(feature = "bindgen")]
#[test]
fn bindgen_is_enabled() {
    println!("bindgen is enabled");
}

#[cfg(not(feature = "bindgen"))]
#[test]
fn bindgen_is_not_enabled() {
    println!("bindgen is not enabled");
}
