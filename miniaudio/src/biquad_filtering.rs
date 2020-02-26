use crate::base::*;
use crate::frames::*;
use miniaudio_sys as sys;

pub struct BiquadConfig(sys::ma_biquad_config);

impl BiquadConfig {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        numerators: [f64; 3],
        denominators: [f64; 3],
    ) -> BiquadConfig {
        unsafe {
            BiquadConfig(sys::ma_biquad_config_init(
                format as _,
                channels,
                numerators[0],
                numerators[1],
                numerators[2],
                denominators[0],
                denominators[1],
                denominators[2],
            ))
        }
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

    /// Get a Biquad Coefficient numerator.
    #[inline]
    pub fn b(&self, index: usize) -> f64 {
        match index {
            0 => self.0.b0,
            1 => self.0.b1,
            2 => self.0.b2,
            _ => panic!("Numerator out of range."),
        }
    }

    /// Set a Biquad Coefficient numerator.
    #[inline]
    pub fn set_b(&mut self, index: usize, value: f64) {
        match index {
            0 => self.0.b0 = value,
            1 => self.0.b1 = value,
            2 => self.0.b2 = value,
            _ => panic!("Numerator out of range."),
        }
    }

    /// Get a Biquad Coefficient denominator.
    #[inline]
    pub fn a(&self, index: usize) -> f64 {
        match index {
            0 => self.0.a0,
            1 => self.0.a1,
            2 => self.0.a2,
            _ => panic!("Denominator out of range."),
        }
    }

    /// Set a Biquad Coefficient denominator.
    #[inline]
    pub fn set_a(&mut self, index: usize, value: f64) {
        match index {
            0 => self.0.a0 = value,
            1 => self.0.a1 = value,
            2 => self.0.a2 = value,
            _ => panic!("Denominator out of range."),
        }
    }
}

pub struct Biquad(sys::ma_biquad);

impl Biquad {
    pub fn new(config: BiquadConfig) -> Result<Biquad, Error> {
        let mut biquad = std::mem::MaybeUninit::uninit();
        unsafe {
            let result = sys::ma_biquad_init(&config.0, biquad.as_mut_ptr());
            map_result!(result, Biquad(biquad.assume_init()))
        }
    }

    pub fn reinit(&mut self, config: BiquadConfig) -> Result<(), Error> {
        let result = unsafe { sys::ma_biquad_reinit(&config.0, &mut self.0) };
        Error::from_c_result(result)
    }

    pub fn process_pcm_frames<'o, 'i, O: Into<FramesMut<'o>>, I: Into<Frames<'i>>>(
        &mut self,
        frames_out: O,
        frames_in: I,
    ) -> Result<(), Error> {
        let frames_out = frames_out.into();
        let frames_in = frames_in.into();

        // FIXME bounds check the frame lengths (???)
        let result = unsafe {
            sys::ma_biquad_process_pcm_frames(
                &mut self.0,
                frames_out.data_ptr as _,
                frames_in.data_ptr as _,
                frames_in.count as _,
            )
        };
        Error::from_c_result(result)
    }

    // FIXME the pointer is not marked as const in the C source even though it could be. I should
    // probably try to get that fixed.
    #[inline]
    pub fn latency(&mut self) -> u32 {
        unsafe { sys::ma_biquad_get_latency(&mut self.0) }
    }
}
