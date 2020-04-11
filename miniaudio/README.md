Mini Audio Rust Bindings
===

[![Build Status](https://github.com/ExPixel/miniaudio-rs/workflows/Tests/badge.svg)](https://github.com/ExPixel/miniaudio-rs/actions?query=workflow%3ATests)
[![crates.io](https://img.shields.io/crates/v/miniaudio.svg?color=orange)](https://crates.io/crates/miniaudio)
[![docs.rs](https://img.shields.io/badge/docs-stable-blue.svg)](https://docs.rs/miniaudio)

Bindings to https://github.com/dr-soft/miniaudio

**
The crate currently lacks documentation, but for the most part the API is very close the the API of the miniaudio C library.
That can be found in the C library's main header file.
**

Building
---
LLVM must be installed in order to generate the bindings, but aside from that everything should __just work__.
Feel free to open an issue here if that is not the case.


Example Usage
---
```rust
//! Enumerating Devices

use miniaudio::Context;

pub fn main() {
    let context = Context::new(&[], None).expect("failed to create context");

    context
        .with_devices(|playback_devices, capture_devices| {
            println!("Playback Devices:");
            for (idx, device) in playback_devices.iter().enumerate() {
                println!("\t{}: {}", idx, device.name());
            }

            println!("Capture Devices:");
            for (idx, device) in capture_devices.iter().enumerate() {
                println!("\t{}: {}", idx, device.name());
            }
        })
        .expect("failed to get devices");
}
```
