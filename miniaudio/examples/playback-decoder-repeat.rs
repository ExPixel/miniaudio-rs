use miniaudio::{Device, DeviceConfig, DeviceType, SyncDecoder};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn main() {
    let file =
        std::fs::File::open("miniaudio/examples/assets/exit.wav").expect("failed to open exit.wav");

    let decoder =
        SyncDecoder::from_read(file, None).expect("failed to initialize decoder from file");

    let mut config = DeviceConfig::new(DeviceType::Playback);
    config.playback_mut().set_format(decoder.output_format());
    config
        .playback_mut()
        .set_channels(decoder.output_channels());
    config.set_sample_rate(decoder.output_sample_rate());

    // This is set to true when a rewind is requested.
    let rewind = Arc::new(AtomicBool::new(false));

    let playback_rewind = Arc::clone(&rewind);
    let playback_decoder = decoder.clone();
    config.set_data_callback(move |_device, output, _frames| {
        if !playback_rewind.load(Ordering::Acquire) {
            let frames = playback_decoder.read_pcm_frames(output);

            // If there were no more frames read, request a rewind.
            if frames == 0 {
                playback_rewind.store(true, Ordering::Release);
            }
        }
    });

    config.set_stop_callback(|_device| {
        println!("Device Stopped.");
    });

    let device = Device::new(None, &config).expect("failed to open playback device");
    device.start().expect("failed to start device");

    println!("Device Backend: {:?}", device.context().backend());

    let (send_shutdown, recv_shutdown) = std::sync::mpsc::channel::<bool>();

    // This time we actually wait for you to press enter on a different thread.
    let wait_thread = std::thread::spawn(move || {
        wait_for_enter();
        send_shutdown
            .send(true)
            .expect("failed to send shutdown request");
    });

    // In here we just loop and rewind th decoder.
    loop {
        // If a rewind was requested, rewind the decoder and reset the flag.
        if rewind.load(Ordering::Acquire) {
            println!("rewind requested...");
            decoder
                .seek_to_pcm_frame(0)
                .expect("error occurred while rewinding");
            rewind.store(false, Ordering::Release);
        }

        match recv_shutdown.try_recv() {
            // Received a request to shutdown.
            Ok(_) => break,

            // Nothing happened so this is a NOP.
            Err(std::sync::mpsc::TryRecvError::Empty) => {}

            // The input wait thread disconnected for some reason so we should bail.
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }

        std::thread::yield_now();
    }

    println!("Shutting Down...");

    wait_thread.join().expect("failed to join wait thread");
}

/// Shows a prompt and waits for input on stdin.
fn wait_for_enter() {
    use std::io::Write;

    println!("Press ENTER/RETURN to exit...");
    // Make sure the line above is displayed:
    std::io::stdout().flush().expect("failed to flush stdout");
    // Just read some random line off of stdin and discard it:
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("failed to wait for line");
}
