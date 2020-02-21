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
    let mut sine_wave = MaybeUninit::<sys::ma_waveform>::uninit();

    // This has to be boxed because it requires a stable address, and Rust's moves basically make
    // that impossible without a heap allocation.
    let mut device = Box::new(MaybeUninit::<sys::ma_device>::uninit());

    sys::ma_waveform_init(
        sys::ma_waveform_type_sine,
        0.2,
        220.0,
        DEVICE_SAMPLE_RATE,
        sine_wave.as_mut_ptr(),
    );
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
    let mut device = transmute::<_, Box<sys::ma_device>>(device);

    println!(
        "Device Name: {}",
        sys::util::cstr_display(&device.playback.name)
    );

    if sys::ma_device_start(&mut *device) != sys::MA_SUCCESS as _ {
        eprintln!("Failed to start playback device.");
        sys::ma_device_uninit(&mut *device);
        return -5;
    }

    wait_for_enter();
    println!("Shutting Down...");

    sys::ma_device_uninit(&mut *device);

    return 0;
}

/// Shows a prompt and waits for input on stdin.
fn wait_for_enter() {
    use std::io::Write;

    print!("Press ENTER/RETURN to exit...");
    // Make sure the line above is displayed:
    std::io::stdout().flush().expect("failed to flush stdout");
    // Just read some random line off of stdin and discard it:
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("failed to wait for line");
}

unsafe extern "C" fn stop_callback(_device_ptr: *mut sys::ma_device) {
    println!("Device Stopped.");
}

unsafe extern "C" fn data_callback(
    device_ptr: *mut sys::ma_device,
    output_ptr: *mut c_void,
    _input_ptr: *const c_void,
    frame_count: u32,
) {
    assert_eq!((*device_ptr).playback.channels, DEVICE_CHANNELS);
    let sine_wave = transmute::<_, *mut sys::ma_waveform>((*device_ptr).pUserData);
    assert_ne!(sine_wave, null_mut());

    sys::ma_waveform_read_pcm_frames(
        sine_wave,
        output_ptr,
        frame_count as _,
        sys::ma_format_f32,
        DEVICE_CHANNELS,
    );
}
