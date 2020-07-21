use crate::lock::{RwLockReadGuard, RwLockWriteGuard, SpinRwLock};
use crate::{Error, Format, FramesMut};
use miniaudio_sys as sys;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
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

/// A decoder with synchronization. This will use a spinlock to synchronize access to the decoder
/// on each function call. The decoder may have multiple readers or one writer.
pub struct SyncDecoder {
    inner: SpinRwLock<RawDecoder>,
}

impl SyncDecoder {
    pub fn from_file(file: &str, config: Option<&DecoderConfig>) -> Result<Arc<Self>, Error> {
        let decoder = Arc::new(MaybeUninit::<SyncDecoder>::uninit());
        let filename = CString::new(file.to_string()).map_err(|_err| Error::InvalidFile)?;

        let result = unsafe {
            sys::ma_decoder_init_file(
                filename.as_ptr() as *const _,
                config.map(|c| &c.0 as *const _).unwrap_or(std::ptr::null()),
                (*Arc::deref(&decoder).as_ptr()).inner.as_ptr() as *const _ as *mut _,
            )
        };

        map_result!(result, unsafe { std::mem::transmute(decoder) })
    }

    pub fn from_memory(data: &[u8], config: Option<&DecoderConfig>) -> Result<Arc<Self>, Error> {
        let decoder = Arc::new(MaybeUninit::<SyncDecoder>::uninit());

        let result = unsafe {
            sys::ma_decoder_init_memory(
                data.as_ptr() as *const _,
                data.len() as _,
                config.map(|c| &c.0 as *const _).unwrap_or(std::ptr::null()),
                (*Arc::deref(&decoder).as_ptr()).inner.as_ptr() as *const _ as *mut _,
            )
        };

        map_result!(result, unsafe { std::mem::transmute(decoder) })
    }

    /// This will block until the lock for the inner decoder is acquired before calling
    /// `read_pcm_frames`.
    #[inline]
    pub fn read_pcm_frames(&self, output: &mut FramesMut) -> u64 {
        self.inner.write().read_pcm_frames(output)
    }

    /// This will immediately return with 0 if the inner decoder is currently locked, if it is not
    /// this will acquire the lock and return the number of frames written.
    #[inline]
    pub fn try_read_pcm_frames(&self, output: &mut FramesMut) -> u64 {
        if let Some(ref mut locked) = self.inner.try_write() {
            locked.read_pcm_frames(output)
        } else {
            0
        }
    }

    #[inline]
    pub fn seek_to_pcm_frame(&self, frame_index: u64) -> Result<(), Error> {
        self.inner.write().seek_to_pcm_frame(frame_index)
    }

    #[inline]
    pub fn length_in_pcm_frames(&self) -> u64 {
        self.inner.write().length_in_pcm_frames()
    }

    #[inline]
    pub fn output_format(&self) -> Format {
        self.inner.read().output_format()
    }

    #[inline]
    pub fn output_channels(&self) -> u32 {
        self.inner.read().output_channels()
    }

    #[inline]
    pub fn output_sample_rate(&self) -> u32 {
        self.inner.read().output_sample_rate()
    }

    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, RawDecoder> {
        self.inner.write()
    }

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<'_, RawDecoder> {
        self.inner.read()
    }

    #[inline]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, RawDecoder>> {
        self.inner.try_write()
    }

    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, RawDecoder>> {
        self.inner.try_read()
    }
}

unsafe impl Send for SyncDecoder {}
unsafe impl Sync for SyncDecoder {}

#[repr(transparent)]
pub struct RawDecoder {
    inner: sys::ma_decoder,
}

impl RawDecoder {
    #[inline]
    pub fn read_pcm_frames(&mut self, output: &mut FramesMut) -> u64 {
        unsafe {
            sys::ma_decoder_read_pcm_frames(
                &self.inner as *const _ as *mut _,
                output.as_mut_ptr() as *mut _,
                output.frame_count() as u64,
            )
        }
    }

    #[inline]
    pub fn length_in_pcm_frames(&mut self) -> u64 {
        unsafe { sys::ma_decoder_get_length_in_pcm_frames(&self.inner as *const _ as *mut _) }
    }

    #[inline]
    pub fn seek_to_pcm_frame(&mut self, frame_index: u64) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_decoder_seek_to_pcm_frame(&self.inner as *const _ as *mut _, frame_index)
        })
    }

    pub fn output_format(&self) -> Format {
        Format::from_c(self.inner.outputFormat)
    }

    pub fn output_channels(&self) -> u32 {
        self.inner.outputChannels as _
    }

    pub fn output_sample_rate(&self) -> u32 {
        self.inner.outputSampleRate as _
    }
}

impl Drop for RawDecoder {
    fn drop(&mut self) {
        Error::from_c_result(unsafe { sys::ma_decoder_uninit(&mut self.inner) })
            .expect("failed to uninit decoder");
    }
}

pub struct Decoder {
    inner: RawDecoder,
}

impl Decoder {
    pub fn from_file(file: &str, config: Option<&DecoderConfig>) -> Result<Box<Self>, Error> {
        let decoder = Box::new(MaybeUninit::<Decoder>::uninit());
        let filename = CString::new(file.to_string()).map_err(|_err| Error::InvalidFile)?;

        let result = unsafe {
            sys::ma_decoder_init_file(
                filename.as_ptr() as *const _,
                config.map(|c| &c.0 as *const _).unwrap_or(std::ptr::null()),
                &(*decoder.as_ptr()).inner as *const _ as *mut _,
            )
        };

        map_result!(result, unsafe { std::mem::transmute(decoder) })
    }

    pub fn from_memory(data: &[u8], config: Option<&DecoderConfig>) -> Result<Box<Self>, Error> {
        let decoder = Box::new(MaybeUninit::<Decoder>::uninit());

        let result = unsafe {
            sys::ma_decoder_init_memory(
                data.as_ptr() as *const _,
                data.len() as _,
                config.map(|c| &c.0 as *const _).unwrap_or(std::ptr::null()),
                &(*decoder.as_ptr()).inner as *const _ as *mut _,
            )
        };

        map_result!(result, unsafe { std::mem::transmute(decoder) })
    }
}

impl Deref for Decoder {
    type Target = RawDecoder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe impl Send for Decoder {}
unsafe impl Sync for Decoder {}
