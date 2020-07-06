use miniaudio::{Decoder, Device, DeviceConfig, DeviceType};

pub fn main() {
    let mut decoder = Decoder::from_file("miniaudio/examples/assets/exit.wav", None).unwrap();
    let mut config = DeviceConfig::new(DeviceType::Playback);
    config.playback_mut().set_format(decoder.output_format());
    config
        .playback_mut()
        .set_channels(decoder.output_channels());
    config.set_sample_rate(decoder.output_sample_rate());

    config.set_data_callback(move |_device, output, _frames| {
        decoder.read_pcm_frames(output);
    });

    config.set_stop_callback(|_device| {
        println!("Device Stopped.");
    });

    let device = Device::new(None, &config).expect("failed to open playback device");
    device.start().expect("failed to start device");

    println!("Device Backend: {:?}", device.context().backend());
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
