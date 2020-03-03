use miniaudio::generation::{Waveform, WaveformConfig, WaveformType};
use miniaudio::{Device, DeviceConfig, DeviceType, Format};

pub const DEVICE_FORMAT: Format = Format::F32;
pub const DEVICE_CHANNELS: u32 = 2;
pub const DEVICE_SAMPLE_RATE: u32 = miniaudio::SAMPLE_RATE_48000;

pub fn main() {
    let sine_wave_config = WaveformConfig::new(
        DEVICE_FORMAT,
        DEVICE_CHANNELS,
        DEVICE_SAMPLE_RATE,
        WaveformType::Sine,
        0.2,
        220.0,
    );
    let mut sine_wave = Waveform::new(&sine_wave_config);

    let mut device_config = DeviceConfig::new(DeviceType::Playback);
    device_config.playback_mut().set_format(DEVICE_FORMAT);
    device_config.playback_mut().set_channels(DEVICE_CHANNELS);
    device_config.set_sample_rate(DEVICE_SAMPLE_RATE);

    device_config.set_data_callback(move |_device, output, _input, _frame_count| {
        let output = miniaudio::as_frames_mut!(output, f32, 2);
        sine_wave.read_pcm_frames(output);
    });

    device_config.set_stop_callback(|_device| {
        println!("Device Stopped.");
    });

    let device = Device::alloc(None, &device_config).expect("failed to open playback device");
    device.start().expect("failed to start device");

    println!(
        "Device Backend: {:?}",
        device
            .owned_context()
            .expect("the context is not owned by the device")
            .backend()
    );
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
