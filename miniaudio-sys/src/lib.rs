#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub mod util;

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "bindgen"), feature = "ma-enable-vorbis"))]
include!("../generated-bindings/bindings-with-vorbis.rs");

#[cfg(all(not(feature = "bindgen"), not(feature = "ma-enable-vorbis")))]
include!("../generated-bindings/bindings.rs");
