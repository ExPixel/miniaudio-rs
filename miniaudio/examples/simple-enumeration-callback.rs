use miniaudio::{Context, DeviceType};

pub fn main() {
    let context = Context::alloc(None, None).expect("failed to create context");

    println!("Devices:");
    unsafe {
        context
            .enumerate_devices(|_, device_type, device_info| {
                let device_type_str = match device_type {
                    DeviceType::Playback => "Playback",
                    DeviceType::Capture => "Capture",
                    DeviceType::Duplex => "Duplex",
                    DeviceType::Loopback => "Loopback",
                };

                println!("\t[{}] {}", device_type_str, device_info.name());

                true
            })
            .expect("failed to enumerate devices");
    }
}
