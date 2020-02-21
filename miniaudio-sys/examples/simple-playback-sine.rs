use miniaudio_sys as sys;
use std::ffi::c_void;
use std::mem::{transmute, MaybeUninit};
use std::ptr::null_mut;

pub const DEVICE_FORMAT: sys::ma_format = sys::ma_format_f32;
pub const DEVICE_CHANNELS: u32 = 2;
pub const DEVICE_SAMPLE_RATE: u32 = 44100;

pub fn main() {
    let status = unsafe { enumerate_devices() };
    std::process::exit(status);
}

pub unsafe fn enumerate_devices() -> i32 {
    let mut sine_wave = MaybeUninit::<sys::ma_sine_wave>::uninit();
    let mut device = MaybeUninit::<sys::ma_device>::uninit();

    sys::ma_sine_wave_init(0.2, 400.0, DEVICE_SAMPLE_RATE, sine_wave.as_mut_ptr());
    let mut sine_wave = sine_wave.assume_init();

    let mut device_config = sys::ma_device_config_init(sys::ma_device_type_playback);

    device_config.playback.format = DEVICE_FORMAT;
    device_config.playback.channels = DEVICE_CHANNELS;
    device_config.sampleRate = DEVICE_SAMPLE_RATE;
    device_config.dataCallback = Some(data_callback);
    device_config.stopCallback = Some(stop_callback);
    device_config.pUserData = transmute(&mut sine_wave);

    if sys::ma_device_init(null_mut(), &device_config, device.as_mut_ptr()) != sys::MA_SUCCESS as _
    {
        eprintln!("Failed to open playback device.");
        return -4;
    }
    let mut device = device.assume_init();

    println!(
        "Device Name: {}",
        sys::util::cstr_display(&device.playback.name)
    );

    if sys::ma_device_start(&mut device) != sys::MA_SUCCESS as _ {
        eprintln!("Failed to start playback device.");
        sys::ma_device_uninit(&mut device);
        return -5;
    }

    println!("Shutting Down...");

    sys::ma_device_uninit(&mut device);

    return 0;
}

unsafe extern "C" fn stop_callback(device_ptr: *mut sys::ma_device) {
    println!("stop-callback");
}

unsafe extern "C" fn data_callback(
    device_ptr: *mut sys::ma_device,
    output_ptr: *mut c_void,
    _input_ptr: *const c_void,
    frame_count: u32,
) {
    println!("data-callback");
    assert_eq!((*device_ptr).playback.channels, DEVICE_CHANNELS);
    let sine_wave = transmute::<_, *mut sys::ma_sine_wave>((*device_ptr).pUserData);
    assert_ne!(sine_wave, null_mut());
    sys::ma_sine_wave_read_pcm_frames(
        sine_wave,
        output_ptr,
        frame_count as _,
        sys::ma_format_f32,
        DEVICE_CHANNELS,
    );
}
