use miniaudio::{Decoder, Device, DeviceConfig, DeviceType};
use std::sync::{Arc, Mutex};

pub fn main() {
    let decoder = Decoder::from_file("miniaudio/examples/assets/exit.wav", None).unwrap();
    println!("Input file length (secs): {:?}", decoder.length());

    let mut config = DeviceConfig::new(DeviceType::Playback);
    config.playback_mut().set_format(decoder.output_format());
    config
        .playback_mut()
        .set_channels(decoder.output_channels());
    config.set_sample_rate(decoder.output_sample_rate());

    // Wrap decoder in Arc/Mutex so we can also use it for seeking later in the code
    let decoder = Arc::new(Mutex::new(decoder));
    let callback_decoder = decoder.clone();
    config.set_data_callback(move |_device, output, _frames| {
        callback_decoder.lock().unwrap().read_pcm_frames(output);
    });

    config.set_stop_callback(|_device| {
        println!("Device Stopped.");
    });

    let device = Device::new(None, &config).expect("failed to open playback device");
    device.start().expect("failed to start device");

    println!("Device Backend: {:?}", device.context().backend());
    print!("Press ENTER/RETURN to play again...");
    wait_for_enter();

    if !decoder.lock().unwrap().seek_to_secs(0.0) {
        println!("Unable to seek");
    }
    print!("Press ENTER/RETURN to exit...");
    wait_for_enter();

    println!("Shutting Down...");
}

/// Shows a prompt and waits for input on stdin.
fn wait_for_enter() {
    use std::io::Write;

    // Make sure the line above is displayed:
    std::io::stdout().flush().expect("failed to flush stdout");
    // Just read some random line off of stdin and discard it:
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("failed to wait for line");
}
