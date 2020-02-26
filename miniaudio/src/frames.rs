use std::marker::PhantomData;

use crate::base::*;

/// A frame is a groups of samples equal to the number of channels. For a stereo stream a frame is 2 samples, a mono frame is 1 sample, a 5.1 surround sound frame
/// is 6 samples, etc. The terms "frame" and "PCM frame" are the same thing in miniaudio. Note that this is different to a compressed frame. If ever miniaudio
/// needs to refer to a compressed frame, such as a FLAC frame, it will always clarify what it's referring to with something like "FLAC frame" or whatnot.
pub struct Frames<'f> {
    /// Pointer to the slice containing frame data.
    pub(crate) data_ptr: *const std::os::raw::c_void,

    /// The format fo the frame data found in `data_ptr`.
    pub(crate) format: Format,

    /// The number of frames contains in `data_ptr`.
    pub(crate) count: usize,

    /// The number of channels contained in each frame in `data_ptr`.
    pub(crate) channels: u32,

    /// For lifetime.
    phantom: PhantomData<&'f ()>,
}

impl<'f> Frames<'f> {
    #[inline]
    pub fn new<T: Sized>(data: &[T], format: Format, channels: u32) -> Frames<'f> {
        assert!(
            std::mem::size_of::<T>() == format.size_in_bytes(),
            "format size in bytes does not match sample size in bytes"
        );

        Frames {
            data_ptr: data.as_ptr() as _,
            format: format,
            count: data.len() / channels as usize,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn new_unchecked<T>(data: &[T], format: Format, channels: u32) -> Frames<'f> {
        Frames {
            data_ptr: data.as_ptr() as _,
            format: format,
            count: data.len() / channels as usize,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_samples<S: FrameSample>(samples: &[S], channels: u32) -> Frames<'f> {
        Self::new(samples, S::FORMAT, channels)
    }

    #[inline]
    pub fn from_ptr<T: Sized>(
        data: *const T,
        frames_count: usize,
        format: Format,
        channels: u32,
    ) -> Frames<'f> {
        assert!(
            std::mem::size_of::<T>() == format.size_in_bytes(),
            "format size in bytes does not match sample size in bytes"
        );

        Frames {
            data_ptr: data as _,
            format: format,
            count: frames_count,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked<T: Sized>(
        data: *const T,
        frames_count: usize,
        format: Format,
        channels: u32,
    ) -> Frames<'f> {
        Frames {
            data_ptr: data as _,
            format: format,
            count: frames_count,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    /// Returns the number of frames that this contains.
    #[inline]
    pub fn frame_count(&self) -> usize {
        self.count
    }

    /// Returns the number of channels per frame.
    #[inline]
    pub fn channels(&self) -> usize {
        self.channels as usize
    }

    /// Returns the number of samples contained in here. This s equivalent to `frame_count` * `channels`
    #[inline]
    pub fn samples(&self) -> usize {
        self.count * self.channels as usize
    }

    /// Returns the byte size of the data contained in here. This is equavalent to `samples` *
    /// sizeof(format)
    #[inline]
    pub fn size_in_bytes(&self) -> usize {
        self.count * (self.channels as usize) * self.format.size_in_bytes()
    }
}

/// A frame is a groups of samples equal to the number of channels. For a stereo stream a frame is 2 samples, a mono frame is 1 sample, a 5.1 surround sound frame
/// is 6 samples, etc. The terms "frame" and "PCM frame" are the same thing in miniaudio. Note that this is different to a compressed frame. If ever miniaudio
/// needs to refer to a compressed frame, such as a FLAC frame, it will always clarify what it's referring to with something like "FLAC frame" or whatnot.
pub struct FramesMut<'f> {
    /// Pointer to the slice containing frame data.
    pub(crate) data_ptr: *mut std::os::raw::c_void,

    /// The format fo the frame data found in `data_ptr`.
    format: Format,

    /// The number of frames contains in `data_ptr`.
    count: usize,

    /// The number of channels contained in each frame in `data_ptr`.
    channels: u32,

    /// For lifetime.
    phantom: PhantomData<&'f ()>,
}

impl<'f> FramesMut<'f> {
    #[inline]
    pub fn new<T: Sized>(data: &mut [T], format: Format, channels: u32) -> FramesMut<'f> {
        assert!(
            std::mem::size_of::<T>() == format.size_in_bytes(),
            "format size in bytes does not match sample size in bytes"
        );

        FramesMut {
            data_ptr: data.as_mut_ptr() as _,
            format: format,
            count: data.len() / channels as usize,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_ptr<T: Sized>(
        data: *mut T,
        frames_count: usize,
        format: Format,
        channels: u32,
    ) -> FramesMut<'f> {
        assert!(
            std::mem::size_of::<T>() == format.size_in_bytes(),
            "format size in bytes does not match sample size in bytes"
        );

        FramesMut {
            data_ptr: data as _,
            format: format,
            count: frames_count,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked<T: Sized>(
        data: *mut T,
        frames_count: usize,
        format: Format,
        channels: u32,
    ) -> FramesMut<'f> {
        FramesMut {
            data_ptr: data as _,
            format: format,
            count: frames_count,
            channels: channels,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_samples<S: FrameSample>(samples: &mut [S], channels: u32) -> FramesMut<'f> {
        Self::new(samples, S::FORMAT, channels)
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    /// Returns the number of frames that this contains.
    #[inline]
    pub fn frame_count(&self) -> usize {
        self.count
    }

    /// Returns the number of channels per frame.
    #[inline]
    pub fn channels(&self) -> usize {
        self.channels as usize
    }

    /// Returns the number of samples contained in here. This s equivalent to `frame_count` * `channels`
    #[inline]
    pub fn samples(&self) -> usize {
        self.count * self.channels as usize
    }

    /// Returns the byte size of the data contained in here. This is equavalent to `samples` *
    /// sizeof(format)
    #[inline]
    pub fn size_in_bytes(&self) -> usize {
        self.count * (self.channels as usize) * self.format.size_in_bytes()
    }
}

impl<'f> From<FramesMut<'f>> for Frames<'f> {
    fn from(mutable: FramesMut<'f>) -> Frames<'f> {
        Frames {
            data_ptr: mutable.data_ptr,
            format: mutable.format,
            count: mutable.count,
            channels: mutable.channels,
            phantom: PhantomData,
        }
    }
}

pub trait FrameSample {
    const FORMAT: Format;
}

impl FrameSample for u8 {
    const FORMAT: Format = Format::U8;
}

impl FrameSample for i16 {
    const FORMAT: Format = Format::S16;
}

impl FrameSample for [i8; 3] {
    const FORMAT: Format = Format::S24;
}

impl FrameSample for i32 {
    const FORMAT: Format = Format::S32;
}

impl FrameSample for f32 {
    const FORMAT: Format = Format::F32;
}
