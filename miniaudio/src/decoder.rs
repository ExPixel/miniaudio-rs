use crate::{Error, Format, FramesMut};
use bitflags::_core::ops::Deref;
use miniaudio_sys as sys;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::sync::Arc;

#[repr(transparent)]
#[derive(Clone)]
pub struct DecoderConfig(sys::ma_decoder_config);

impl DecoderConfig {
    #[inline]
    pub fn new(format: Format, output_channels: u32, output_sample_rate: u32) -> Self {
        DecoderConfig(unsafe {
            sys::ma_decoder_config_init(format as _, output_channels as _, output_sample_rate as _)
        })
    }
}

#[derive(Clone)]
pub struct Decoder(Arc<RawDecoder>);

impl Decoder {
    #[inline]
    pub fn from_file(file: &str, config: Option<&DecoderConfig>) -> Result<Decoder, Error> {
        RawDecoder::from_file(file, config).map(Decoder)
    }

    #[inline]
    pub fn from_memory(data: &[u8], config: Option<&DecoderConfig>) -> Result<Decoder, Error> {
        RawDecoder::from_memory(data, config).map(Decoder)
    }

    #[inline]
    pub fn output_format(&self) -> Format {
        Format::from_c(self.raw().outputFormat)
    }

    #[inline]
    pub fn output_channels(&self) -> u32 {
        self.raw().outputChannels
    }

    #[inline]
    pub fn output_sample_rate(&self) -> u32 {
        self.raw().outputSampleRate
    }

    #[inline]
    pub fn read_pcm_frames(&mut self, output: &mut FramesMut) -> u64 {
        unsafe {
            sys::ma_decoder_read_pcm_frames(
                Arc::deref(&self.0) as *const _ as *mut _,
                output.as_mut_ptr() as *mut _,
                output.frame_count() as u64,
            )
        }
    }

    #[inline]
    pub fn length_in_pcm_frames(&mut self) -> u64 {
        unsafe {
            sys::ma_decoder_get_length_in_pcm_frames(Arc::deref(&self.0) as *const _ as *mut _)
        }
    }

    #[inline]
    pub fn seek_to_pcm_frame(&mut self, frame_index: u64) -> Result<(), Error> {
        let result = unsafe {
            sys::ma_decoder_seek_to_pcm_frame(
                Arc::deref(&self.0) as *const _ as *mut _,
                frame_index,
            )
        };

        map_result!(result, ())
    }

    #[inline]
    fn raw(&self) -> &sys::ma_decoder {
        &(self.0).0
    }
}

impl std::ops::Deref for Decoder {
    type Target = RawDecoder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct RawDecoder(sys::ma_decoder);

unsafe impl Send for RawDecoder {}
unsafe impl Sync for RawDecoder {}

impl RawDecoder {
    fn from_file(file: &str, config: Option<&DecoderConfig>) -> Result<Arc<Self>, Error> {
        let decoder = Arc::new(MaybeUninit::<RawDecoder>::uninit());
        let filename = CString::new(file.to_string()).map_err(|_err| Error::InvalidFile)?;

        let result = unsafe {
            sys::ma_decoder_init_file(
                filename.as_ptr() as *const _,
                config.map(|c| &c.0 as *const _).unwrap_or(std::ptr::null()),
                Arc::deref(&decoder).as_ptr() as *mut _,
            )
        };

        map_result!(result, unsafe { std::mem::transmute(decoder) })
    }

    fn from_memory(data: &[u8], config: Option<&DecoderConfig>) -> Result<Arc<Self>, Error> {
        let decoder = Arc::new(MaybeUninit::<RawDecoder>::uninit());

        let result = unsafe {
            sys::ma_decoder_init_memory(
                data.as_ptr() as *const _,
                data.len() as _,
                config.map(|c| &c.0 as *const _).unwrap_or(std::ptr::null()),
                Arc::deref(&decoder).as_ptr() as *mut _,
            )
        };

        map_result!(result, unsafe { std::mem::transmute(decoder) })
    }
}

impl Drop for RawDecoder {
    fn drop(&mut self) {
        Error::from_c_result(unsafe { sys::ma_decoder_uninit(&mut self.0) })
            .expect("failed to uninit decoder");
    }
}
