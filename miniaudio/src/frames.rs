use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::base::*;

/// A frame is a group of samples equal to the number of channels out of an output or input stream
/// of samples. For a stereo stream (e.g. left/right channels) a frame is 2 samples, a mono stream
/// has a frame size of 1 sample, and a 5.1 surround sound stream is 6 samples per frame. The terms
/// "frame" and "PCM frame" are the same thing in miniaudio. Note that this is different from a
/// compressed frame.
pub struct Frame<'d, T: Sized> {
    format: Format,
    data: *const T,

    /// Phantom data used for describing the lifetime of the data.
    _lifetime_data: PhantomData<&'d T>,
}

/// A container with multiple frames.
pub struct Frames<'d, T: Sized> {
    /// The format of each sample contained in `data`.
    format: Format,

    /// The number of channels that this frame has per frame.
    channels: usize,

    /// The number of frames contained in `data`.
    count: usize,

    /// The samples organized as an array of frames (each frame being an array of samples per
    /// channel) laid out contiguously.
    data: *const T,

    /// Phantom data used for describing the lifetime of the data.
    _lifetime_data: PhantomData<&'d T>,
}

impl<'d, T: Sized> Frames<'d, T> {
    pub fn frame<'s: 'd>(&'s self, index: usize) -> Frame<'s, T> {
        todo!();
    }
}

// /// A frame is a groups of samples equal to the number of channels. For a stereo stream a frame is 2 samples, a mono frame is 1 sample, a 5.1 surround sound frame
// /// is 6 samples, etc. The terms "frame" and "PCM frame" are the same thing in miniaudio. Note that this is different to a compressed frame. If ever miniaudio
// /// needs to refer to a compressed frame, such as a FLAC frame, it will always clarify what it's referring to with something like "FLAC frame" or whatnot.
// pub struct Frames<'f, T: Sized> {
//     /// Pointer to the slice containing frame data.
//     pub(crate) data_ptr: *const std::os::raw::c_void,

//     /// The format fo the frame data found in `data_ptr`.
//     pub(crate) format: Format,

//     /// The number of frames contains in `data_ptr`.
//     pub(crate) count: usize,

//     /// The number of channels contained in each frame in `data_ptr`.
//     pub(crate) channels: u32,

//     /// For lifetime.
//     phantom: PhantomData<&'f T>,
// }

// impl<'f, T> Frames<'f, T> {
//     #[inline]
//     pub fn new(data: &[T], format: Format, channels: u32) -> Frames<'f, T> {
//         assert!(
//             std::mem::size_of::<T>() == format.size_in_bytes(),
//             "format size in bytes does not match sample size in bytes"
//         );

//         Frames {
//             data_ptr: data.as_ptr() as _,
//             format: format,
//             count: data.len() / channels as usize,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     #[inline]
//     pub unsafe fn new_unchecked(data: &[T], format: Format, channels: u32) -> Frames<'f, T> {
//         Frames {
//             data_ptr: data.as_ptr() as _,
//             format: format,
//             count: data.len() / channels as usize,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     #[inline]
//     pub fn from_ptr(
//         data: *const T,
//         frames_count: usize,
//         format: Format,
//         channels: u32,
//     ) -> Frames<'f, T> {
//         assert!(
//             std::mem::size_of::<T>() == format.size_in_bytes(),
//             "format size in bytes does not match sample size in bytes"
//         );

//         Frames {
//             data_ptr: data as _,
//             format: format,
//             count: frames_count,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     #[inline]
//     pub unsafe fn from_ptr_unchecked(
//         data: *const T,
//         frames_count: usize,
//         format: Format,
//         channels: u32,
//     ) -> Frames<'f, T> {
//         Frames {
//             data_ptr: data as _,
//             format: format,
//             count: frames_count,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     pub fn data_ptr(&self) -> *const std::os::raw::c_void {
//         self.data_ptr
//     }

//     #[inline]
//     pub fn format(&self) -> Format {
//         self.format
//     }

//     /// Returns the number of frames that this contains.
//     #[inline]
//     pub fn frame_count(&self) -> usize {
//         self.count
//     }

//     /// Returns the number of channels per frame.
//     #[inline]
//     pub fn channels(&self) -> usize {
//         self.channels as usize
//     }

//     /// Returns the number of samples contained in here. This s equivalent to `frame_count` * `channels`
//     #[inline]
//     pub fn samples(&self) -> usize {
//         self.count * self.channels as usize
//     }

//     /// Returns the byte size of the data contained in here. This is equavalent to `samples` *
//     /// sizeof(format)
//     #[inline]
//     pub fn size_in_bytes(&self) -> usize {
//         self.count * (self.channels as usize) * self.format.size_in_bytes()
//     }
// }

// /// A frame is a groups of samples equal to the number of channels. For a stereo stream a frame is 2 samples, a mono frame is 1 sample, a 5.1 surround sound frame
// /// is 6 samples, etc. The terms "frame" and "PCM frame" are the same thing in miniaudio. Note that this is different to a compressed frame. If ever miniaudio
// /// needs to refer to a compressed frame, such as a FLAC frame, it will always clarify what it's referring to with something like "FLAC frame" or whatnot.
// pub struct FramesMut<'f, T: Sized> {
//     /// Pointer to the slice containing frame data.
//     pub(crate) data_ptr: *mut std::os::raw::c_void,

//     /// The format fo the frame data found in `data_ptr`.
//     format: Format,

//     /// The number of frames contains in `data_ptr`.
//     count: usize,

//     /// The number of channels contained in each frame in `data_ptr`.
//     channels: u32,

//     /// For lifetime.
//     phantom: PhantomData<&'f T>,
// }

// impl<'f, T: Sized> FramesMut<'f, T> {
//     #[inline]
//     pub fn new(data: &mut [T], format: Format, channels: u32) -> FramesMut<'f, T> {
//         assert!(
//             std::mem::size_of::<T>() == format.size_in_bytes(),
//             "format size in bytes does not match sample size in bytes"
//         );

//         FramesMut {
//             data_ptr: data.as_mut_ptr() as _,
//             format: format,
//             count: data.len() / channels as usize,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     pub fn data_ptr(&self) -> *const std::os::raw::c_void {
//         self.data_ptr
//     }

//     pub fn from_nonnull(
//         data: NonNull<T>,
//         frames_count: usize,
//         format: Format,
//         channels: u32,
//     ) -> FramesMut<'f, T> {
//         assert!(
//             std::mem::size_of::<T>() == format.size_in_bytes(),
//             "format size in bytes does not match sample size in bytes"
//         );

//         FramesMut {
//             data_ptr: data.as_ptr() as _,
//             format: format,
//             count: frames_count,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     #[inline]
//     pub fn from_ptr(
//         data: *mut T,
//         frames_count: usize,
//         format: Format,
//         channels: u32,
//     ) -> FramesMut<'f, T> {
//         assert!(
//             std::mem::size_of::<T>() == format.size_in_bytes(),
//             "format size in bytes does not match sample size in bytes"
//         );

//         FramesMut {
//             data_ptr: data as _,
//             format: format,
//             count: frames_count,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     #[inline]
//     pub unsafe fn from_ptr_unchecked(
//         data: *mut T,
//         frames_count: usize,
//         format: Format,
//         channels: u32,
//     ) -> FramesMut<'f, T> {
//         FramesMut {
//             data_ptr: data as _,
//             format: format,
//             count: frames_count,
//             channels: channels,
//             phantom: std::marker::PhantomData,
//         }
//     }

//     /// Writes a sample at a given (frame, channel) offset.
//     #[inline]
//     pub fn write_sample(&mut self, frame: usize, channel: usize, sample: T) {
//         let offset = (frame * self.channels as usize) + channel;
//         unsafe { *self.data_ptr.cast::<T>().add(offset) = sample };
//     }

//     #[inline]
//     pub fn format(&self) -> Format {
//         self.format
//     }

//     /// Returns the number of frames that this contains.
//     #[inline]
//     pub fn frame_count(&self) -> usize {
//         self.count
//     }

//     /// Returns the number of channels per frame.
//     #[inline]
//     pub fn channels(&self) -> usize {
//         self.channels as usize
//     }

//     /// Returns the number of samples contained in here. This s equivalent to `frame_count` * `channels`
//     #[inline]
//     pub fn samples(&self) -> usize {
//         self.count * self.channels as usize
//     }

//     /// Returns the byte size of the data contained in here. This is equavalent to `samples` *
//     /// sizeof(format)
//     #[inline]
//     pub fn size_in_bytes(&self) -> usize {
//         self.count * (self.channels as usize) * self.format.size_in_bytes()
//     }
// }

// impl<'f, T> From<FramesMut<'f, T>> for Frames<'f, T> {
//     fn from(mutable: FramesMut<'f, T>) -> Frames<'f, T> {
//         Frames {
//             data_ptr: mutable.data_ptr,
//             format: mutable.format,
//             count: mutable.count,
//             channels: mutable.channels,
//             phantom: PhantomData,
//         }
//     }
// }

// pub trait FrameSample {
//     const FORMAT: Format;
// }

// impl FrameSample for u8 {
//     const FORMAT: Format = Format::U8;
// }

// impl FrameSample for i16 {
//     const FORMAT: Format = Format::S16;
// }

// impl FrameSample for [i8; 3] {
//     const FORMAT: Format = Format::S24;
// }

// impl FrameSample for i32 {
//     const FORMAT: Format = Format::S32;
// }

// impl FrameSample for f32 {
//     const FORMAT: Format = Format::F32;
// }
