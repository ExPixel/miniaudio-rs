#[cfg(test)]
mod tests {
    use miniaudio_sys as sys;

    #[test]
    fn it_works() {
        println!("WORKIN");

        let config = unsafe { sys::ma_format_converter_config_init_new() };
        println!("config: {:?}", config);
        let mut converter: sys::FormatConverter = sys::FormatConverter::default();
        unsafe { sys::ma_format_converter_init(&config, &mut converter) };
        println!("converter: {:?}", converter);
    }
}
