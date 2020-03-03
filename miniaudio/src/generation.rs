use crate::base::{from_bool32, to_bool32, Format};
use crate::frames::{Frames, Sample};
use miniaudio_sys as sys;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WaveformType {
    Sine = sys::ma_waveform_type_sine as _,
    Square = sys::ma_waveform_type_square as _,
    Triangle = sys::ma_waveform_type_triangle as _,
    Sawtooth = sys::ma_waveform_type_sawtooth as _,
}
impl_from_c!(WaveformType, sys::ma_waveform_type);

#[repr(transparent)]
#[derive(Clone)]
pub struct WaveformConfig(sys::ma_waveform_config);

impl WaveformConfig {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        waveform_type: WaveformType,
        amplitude: f64,
        frequency: f64,
    ) -> WaveformConfig {
        WaveformConfig(unsafe {
            sys::ma_waveform_config_init(
                format as _,
                channels,
                sample_rate,
                waveform_type as _,
                amplitude,
                frequency,
            )
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.0.sampleRate = sample_rate;
    }

    #[inline]
    pub fn waveform_type(&self) -> WaveformType {
        WaveformType::from_c(self.0.type_)
    }

    #[inline]
    pub fn amplitude(&self) -> f64 {
        self.0.amplitude
    }

    #[inline]
    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.0.amplitude = amplitude;
    }

    #[inline]
    pub fn frequency(&self) -> f64 {
        self.0.frequency
    }

    #[inline]
    pub fn set_frequency(&mut self, frequency: f64) {
        self.0.frequency = frequency;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Waveform(sys::ma_waveform);

impl Waveform {
    #[inline]
    pub fn new(config: &WaveformConfig) -> Waveform {
        let mut waveform = std::mem::MaybeUninit::<Waveform>::uninit();

        // NOTE: This only really fails if config is NULL which can't happen in Rust.
        unsafe {
            sys::ma_waveform_init(
                config as *const WaveformConfig as *const _,
                waveform.as_mut_ptr() as *mut _,
            );
            waveform.assume_init()
        }
    }

    #[inline]
    pub fn read_pcm_frames<S: Sample + Copy + Sized, F: Sized + Copy>(
        &mut self,
        output: &mut Frames<S, F>,
    ) -> u64 {
        assert!(
            S::format() == self.config().format(),
            "output format not the same as waveform format"
        );

        unsafe {
            sys::ma_waveform_read_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                output.count() as u64,
            )
        }
    }

    #[inline]
    pub fn config(&mut self) -> &WaveformConfig {
        unsafe {
            (&self.0.config as *const sys::ma_waveform_config)
                .cast::<WaveformConfig>()
                .as_ref()
                .unwrap()
        }
    }

    #[inline]
    pub fn advance(&self) -> f64 {
        self.0.advance
    }

    #[inline]
    pub fn set_advance(&mut self, advance: f64) {
        self.0.advance = advance;
    }

    #[inline]
    pub fn time(&self) -> f64 {
        self.0.time
    }

    #[inline]
    pub fn set_time(&mut self, time: f64) {
        self.0.time = time;
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct LCG(sys::ma_lcg);

impl LCG {
    #[inline]
    pub fn state(&self) -> i32 {
        self.0.state
    }

    #[inline]
    pub fn set_state(&mut self, state: i32) {
        self.0.state = state;
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NoiseType {
    White = sys::ma_noise_type_white as _,
    Pink = sys::ma_noise_type_pink as _,
    Brownian = sys::ma_noise_type_brownian as _,
}
impl_from_c!(NoiseType, sys::ma_noise_type);

#[repr(transparent)]
#[derive(Clone)]
pub struct NoiseConfig(sys::ma_noise_config);

impl NoiseConfig {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        noise_type: NoiseType,
        seed: i32,
        amplitude: f64,
    ) -> NoiseConfig {
        NoiseConfig(unsafe {
            sys::ma_noise_config_init(format as _, channels, noise_type as _, seed, amplitude)
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    #[inline]
    pub fn noise_type(&self) -> NoiseType {
        NoiseType::from_c(self.0.type_)
    }

    #[inline]
    pub fn set_noise_type(&mut self, noise_type: NoiseType) {
        self.0.type_ = noise_type as _;
    }

    #[inline]
    pub fn seed(&self) -> i32 {
        self.0.seed
    }

    #[inline]
    pub fn set_seed(&mut self, seed: i32) {
        self.0.seed = seed;
    }

    #[inline]
    pub fn amplitude(&self) -> f64 {
        self.0.amplitude
    }

    #[inline]
    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.0.amplitude = amplitude;
    }

    #[inline]
    pub fn duplicate_channels(&self) -> bool {
        from_bool32(self.0.duplicateChannels)
    }

    #[inline]
    pub fn set_duplicate_channels(&mut self, duplicate: bool) {
        self.0.duplicateChannels = to_bool32(duplicate);
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Noise(sys::ma_noise);

impl Noise {
    pub fn new(config: &NoiseConfig) -> Noise {
        let mut noise = std::mem::MaybeUninit::<Noise>::uninit();
        // NOTE: This only really fails if config is NULL which can't happen in Rust.
        unsafe {
            sys::ma_noise_init(
                config as *const NoiseConfig as *const _,
                noise.as_mut_ptr() as *mut _,
            );
            noise.assume_init()
        }
    }

    pub fn config(&mut self) -> &NoiseConfig {
        unsafe {
            (&self.0.config as *const sys::ma_noise_config)
                .cast::<NoiseConfig>()
                .as_ref()
                .unwrap()
        }
    }

    pub fn read_pcm_frames<S: Sample + Copy + Sized, F: Sized + Copy>(
        &mut self,
        output: &mut Frames<S, F>,
    ) -> u64 {
        assert!(
            S::format() == self.config().format(),
            "output format not the same as waveform format"
        );

        unsafe {
            sys::ma_noise_read_pcm_frames(
                &mut self.0 as *mut _,
                output.frames_ptr_mut() as *mut _,
                output.count() as u64,
            )
        }
    }
}
