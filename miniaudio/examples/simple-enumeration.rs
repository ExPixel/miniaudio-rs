use miniaudio::{Context, DeviceId, DeviceType, ShareMode};

pub fn main() {
    let context = Context::alloc(None, None).expect("failed to create context");

    context
        .with_devices(|playback_devices, capture_devices| {
            println!("Playback Devices:");
            for (idx, device) in playback_devices.iter().enumerate() {
                println!("\t{}: {}", idx, device.name());
                print_device_info(&context, DeviceType::Playback, device.id());
            }

            println!("Capture Devices:");
            for (idx, device) in capture_devices.iter().enumerate() {
                println!("\t{}: {}", idx, device.name());
                print_device_info(&context, DeviceType::Capture, device.id());
            }
        })
        .expect("failed to get devices");
}

pub fn print_device_info(context: &Context, device_type: DeviceType, device_id: &DeviceId) {
    // This can fail, so we have to check the result.
    let info = match context.get_device_info(device_type, device_id, ShareMode::Shared) {
        Ok(info) => info,
        Err(err) => {
            eprintln!("\t\tfailed to get device info: {}", err);
            return;
        }
    };

    println!(
        "\t\tSample Rate: {}-{}Hz",
        info.min_sample_rate(),
        info.max_sample_rate()
    );

    println!(
        "\t\tChannels: {}-{}",
        info.min_channels(),
        info.max_channels()
    );
}
