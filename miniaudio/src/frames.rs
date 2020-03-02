use std::marker::PhantomData;

use crate::base::*;

// FIXME when const generics becomes stable use that instead
// so that we can just have Frames<const channels: u32, SampleType>
/// Contains multiple frames.
///
/// A frame is a group of samples equal to the number of channels out of an output or input stream
/// of samples. For a stereo stream (e.g. left/right channels) a frame is 2 samples, a mono stream
/// has a frame size of 1 sample, and a 5.1 surround sound stream is 6 samples per frame. The terms
/// "frame" and "PCM frame" are the same thing in miniaudio. Note that this is different from a
/// compressed frame.
pub struct Frames<SampleType: Sample + Copy + Sized, FrameType: Copy + Sized> {
    _sample_phantom: PhantomData<SampleType>,
    data: [FrameType],
}

impl<SampleType: Sample + Copy + Sized, FrameType: Copy + Sized> Frames<SampleType, FrameType> {
    pub fn new<'d>(data: &'d [u8]) -> &'d Frames<SampleType, FrameType> {
        // FIXME I should probably assert here that the size of FrameType is a multiple of the
        // size of SampleType.
        //
        // FIXME should also add some checks to make sure that FrameType fits into the data byte
        // slice.
        unsafe {
            std::mem::transmute::<_, &'d Frames<SampleType, FrameType>>(
                std::slice::from_raw_parts::<'d, FrameType>(
                    data.as_ptr().cast::<FrameType>(),
                    data.len() / std::mem::size_of::<FrameType>(),
                ),
            )
        }
    }

    pub fn new_mut<'d>(data: &'d mut [u8]) -> &'d mut Frames<SampleType, FrameType> {
        // FIXME I should probably assert here that the size of FrameType is a multiple of the
        // size of SampleType.
        //
        // FIXME should also add some checks to make sure that FrameType fits into the data byte
        // slice.
        unsafe {
            std::mem::transmute::<_, &'d mut Frames<SampleType, FrameType>>(
                std::slice::from_raw_parts_mut::<'d, FrameType>(
                    data.as_mut_ptr().cast::<FrameType>(),
                    data.len() / std::mem::size_of::<FrameType>(),
                ),
            )
        }
    }

    /// Returns the number of frames contained in here.
    #[inline]
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of samples contained.
    #[inline]
    pub fn sample_count(&self) -> usize {
        self.data.len() * self.channel_count()
    }

    /// Returns the number of channels contained in each frame.
    #[inline(always)]
    pub fn channel_count(&self) -> usize {
        std::mem::size_of::<FrameType>() / std::mem::size_of::<SampleType>()
    }

    /// Returns a reference to the frame at the given index.
    #[inline]
    pub fn frame(&self, index: usize) -> &FrameType {
        &self.data[index]
    }

    /// Returns a mutable reference to the frame at the given index.
    #[inline]
    pub fn frame_mut(&mut self, index: usize) -> &mut FrameType {
        &mut self.data[index]
    }

    /// Returns the value of sample for a channel at a given frame.
    #[inline]
    pub fn sample(&self, frame_index: usize, channel_index: usize) -> &SampleType {
        let frame_ref = self.frame(frame_index);
        let frame_ptr = frame_ref as *const FrameType;
        let samples = frame_ptr as *const SampleType;
        unsafe { samples.add(channel_index).as_ref().unwrap() }
    }

    /// Returns a mutable reference to a sample for a channel at a given frame.
    #[inline]
    pub fn sample_mut(&mut self, frame_index: usize, channel_index: usize) -> &mut SampleType {
        let frame_ref = self.frame_mut(frame_index);
        let frame_ptr = frame_ref as *mut FrameType;
        let samples = frame_ptr as *mut SampleType;
        unsafe { samples.add(channel_index).as_mut().unwrap() }
    }

    #[inline]
    pub fn frames_ptr(&self) -> *const FrameType {
        self.data.as_ptr()
    }

    #[inline]
    pub fn frames_ptr_mut(&mut self) -> *mut FrameType {
        self.data.as_mut_ptr()
    }

    #[inline]
    pub fn samples_ptr(&self) -> *const SampleType {
        self.data.as_ptr().cast()
    }

    #[inline]
    pub fn samples_ptr_mut(&mut self) -> *mut SampleType {
        self.data.as_mut_ptr().cast()
    }
}

/// The type of a sample which corresponds to a `Format`.
pub trait Sample {
    fn format() -> Format;
}

impl Sample for u8 {
    fn format() -> Format {
        Format::U8
    }
}

impl Sample for i16 {
    fn format() -> Format {
        Format::S16
    }
}

// ## NOTE
// Can't implement this for a tuple as well because there are no guarantees for tuple alignment.
// Arrays are however guaranteed to be packed and have the same alignment as the type that they are
// an array of.
impl Sample for [u8; 3] {
    fn format() -> Format {
        Format::S24
    }
}

impl Sample for i32 {
    fn format() -> Format {
        Format::S32
    }
}

impl Sample for f32 {
    fn format() -> Format {
        Format::F32
    }
}
