use miniaudio_sys as sys;
use std::mem::MaybeUninit;

pub fn main() {
    let status = unsafe { enumerate_devices() };
    std::process::exit(status);
}

pub unsafe fn enumerate_devices() -> i32 {
    use std::ptr::null;

    let mut context = MaybeUninit::<sys::ma_context>::uninit();
    if sys::ma_context_init(null(), 0, null(), context.as_mut_ptr()) != sys::MA_SUCCESS as _ {
        eprintln!("Failed to initialize context.");
        return -2;
    }
    let mut context = context.assume_init();

    let mut playback_device_infos = MaybeUninit::<*mut sys::ma_device_info>::uninit();
    let mut playback_device_count = 0u32;

    let mut capture_device_infos = MaybeUninit::<*mut sys::ma_device_info>::uninit();
    let mut capture_device_count = 032;

    let result = sys::ma_context_get_devices(
        &mut context,
        playback_device_infos.as_mut_ptr(),
        &mut playback_device_count,
        capture_device_infos.as_mut_ptr(),
        &mut capture_device_count,
    );

    if result != sys::MA_SUCCESS as _ {
        eprintln!("Failed to retrieve device information.");
        return -3;
    }

    let playback_device_infos = playback_device_infos.assume_init();
    let capture_device_infos = capture_device_infos.assume_init();

    println!("Playback Devices:");
    for device_idx in 0..playback_device_count {
        let device = playback_device_infos.add(device_idx as usize);
        let name = sys::util::cstr_display(&(*device).name);
        println!("\t{}: {}", device_idx, name);
    }

    println!("Capture Devices:");
    for device_idx in 0..capture_device_count {
        let device = capture_device_infos.add(device_idx as usize);
        let name = sys::util::cstr_display(&(*device).name);
        println!("\t{}: {}", device_idx, name);
    }

    sys::ma_context_uninit(&mut context);

    return 0;
}
