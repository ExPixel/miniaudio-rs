use miniaudio::{Device, DeviceConfig, DeviceType, Format, Waveform, WaveformConfig, WaveformType};
use std::ptr::NonNull;

pub const DEVICE_FORMAT: Format = Format::F32;
pub const DEVICE_CHANNELS: u32 = 2;
pub const DEVICE_SAMPLE_RATE: u32 = 44100;

pub fn main() {
    let sine_wave_config = WaveformConfig::new(
        DEVICE_FORMAT,
        DEVICE_CHANNELS,
        DEVICE_SAMPLE_RATE,
        WaveformType::Sine,
        0.2,
        220.0,
    );
    let sine_wave = Waveform::new(&sine_wave_config);

    let mut device_config = DeviceConfig::new(DeviceType::Playback);
    device_config.playback_mut().set_format(DEVICE_FORMAT);
    device_config.playback_mut().set_channels(DEVICE_CHANNELS);
    device_config.set_sample_rate(DEVICE_SAMPLE_RATE);
    device_config.set_data_callback(Some(device_data_callback));
    device_config.set_stop_callback(Some(device_stop_callback));
    device_config.set_user_data(sine_wave);

    let device = Device::alloc(None, &device_config).expect("failed to open playback device");
    device.start().expect("failed to start device");

    wait_for_enter();
    println!("Shutting Down...");
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

extern "C" fn device_data_callback(
    device: NonNull<Device>,
    output: Option<NonNull<()>>,
    _input: Option<NonNull<()>>,
    frame_count: u32,
) {
    let output = output.unwrap();
    let waveform = unsafe { (*device.as_ptr()).user_data_ref::<Waveform>() }.unwrap();
    waveform.read_pcm_frames(output, frame_count as u64);
}

extern "C" fn device_stop_callback(_device: NonNull<Device>) {
    println!("Device Stopped.");
}
