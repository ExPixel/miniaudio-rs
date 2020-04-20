use miniaudio::{Device, DeviceConfig, DeviceType, Format, FramesMut};
use miniaudio::{Waveform, WaveformConfig, WaveformType};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub type DeviceFormatType = f32;
pub const DEVICE_FORMAT: Format = Format::F32;
pub const DEVICE_CHANNELS: u32 = 2;
pub const DEVICE_SAMPLE_RATE: u32 = miniaudio::SAMPLE_RATE_48000;
pub const SUBBUFFER_LEN: usize = 1024;
pub const SUBBUFFER_COUNT: usize = 16;

pub fn main() {
    // Create a new ring buffer with multiple subbuffers that can be exchanged between the producer
    // and the consumer.
    let (send, recv) = miniaudio::ring_buffer::<f32>(SUBBUFFER_LEN, SUBBUFFER_COUNT)
        .expect("failed to create ring buffer");

    //  Shutdown starts as true, and will be set to false when the producer thread is ready.
    //  This way we can wait until samples are being produced.
    let shutdown_producer = Arc::new(AtomicBool::new(true));

    // This is just a clone of the ref counted atomic bool that we're going to pass into the
    // producer thread. It will be shared with the main thread.
    let shutdown_producer_check = Arc::clone(&shutdown_producer);

    println!("Waiting for producer thread...");

    let producer_thread = std::thread::spawn(move || {
        use std::io::Write;

        // Write this message and make sure it's flushed before we signal that the producer thread
        // is ready.
        println!("Running producer thread.");
        std::io::stdout().flush().expect("failed to flush stdout");

        shutdown_producer_check.store(false, std::sync::atomic::Ordering::Release);

        let sine_wave_config = WaveformConfig::new(
            DEVICE_FORMAT,
            DEVICE_CHANNELS,
            DEVICE_SAMPLE_RATE,
            WaveformType::Sine,
            0.2,
            220.0,
        );
        let mut sine_wave = Waveform::new(&sine_wave_config);

        loop {
            // We always just try to fill the entire buffer with samples:
            send.write_with(SUBBUFFER_LEN, |buf| {
                sine_wave.read_pcm_frames(&mut FramesMut::wrap(
                    buf,
                    DEVICE_FORMAT,
                    DEVICE_CHANNELS,
                ));
            });

            if shutdown_producer_check.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
        }
        println!("Shutting down producer thread...");
    });

    // Spin lock >:(
    // Wait for the producer thread to start before showing anything:
    while shutdown_producer.load(std::sync::atomic::Ordering::Acquire) {
        std::thread::yield_now();
    }

    let mut device_config = DeviceConfig::new(DeviceType::Playback);
    device_config.playback_mut().set_format(DEVICE_FORMAT);
    device_config.playback_mut().set_channels(DEVICE_CHANNELS);
    device_config.set_sample_rate(DEVICE_SAMPLE_RATE);

    let mut last_sample = 0.0f32;
    device_config.set_data_callback(move |_device, output, _input| {
        let samples = output.as_samples_mut::<f32>();

        // Here we try reading at most 8 subbuffers to attempt to read enough samples to
        // fill the playback output buffer. We don't allow infinite attempts because we can't be
        // sure how long that would take.
        let mut read_count = 0;
        let mut attempts = 0;
        while read_count < samples.len() && attempts < 8 {
            read_count += recv.read(&mut samples[read_count..]);
            attempts += 1;
        }

        // If we read anything, update the last sample.
        if read_count > 0 {
            last_sample = samples[read_count - 1];
        }

        // If we're starved, just repeat the last sample on all channels:
        (&mut samples[read_count..])
            .iter_mut()
            .for_each(|s| *s = last_sample);
    });

    device_config.set_stop_callback(|_device| {
        println!("Device Stopped.");
    });

    // We start the device after sample production begins.
    let device = Device::new(None, &device_config).expect("failed to open playback device");
    device.start().expect("failed to start device");

    println!("Device Backend: {:?}", device.context().backend());
    wait_for_enter();
    println!("Shutting Down...");

    // Wait for the producer thread to read that it should shutdown and join on it.
    shutdown_producer.store(true, std::sync::atomic::Ordering::Release);
    producer_thread
        .join()
        .expect("failed to join on producer thread");
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
