use crate::base::*;

pub struct Frames<'s> {
    data: &'s [u8],
    format: Format,
    channels: u32,
}

impl<'s> Frames<'s> {
    pub fn wrap(data: &'s [u8], format: Format, channels: u32) -> Frames<'s> {
        Frames {
            data,
            format,
            channels,
        }
    }

    pub(crate) fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of frames contained.
    pub fn frame_count(&self) -> usize {
        self.sample_count() / self.channels as usize
    }

    /// Returns the number of samples contained.
    pub fn sample_count(&self) -> usize {
        self.data.len() / self.format.size_in_bytes()
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }
}

pub struct FramesMut<'s> {
    data: &'s mut [u8],
    format: Format,
    channels: u32,
}

impl<'s> FramesMut<'s> {
    pub fn wrap(data: &'s mut [u8], format: Format, channels: u32) -> FramesMut<'s> {
        FramesMut {
            data,
            format,
            channels,
        }
    }

    // pub(crate) fn as_ptr(&self) -> *const u8 {
    //     self.data.as_ptr()
    // }

    pub(crate) fn as_mut_ptr(&self) -> *mut u8 {
        self.data.as_ptr() as *mut u8
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data
    }

    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of frames contained.
    pub fn frame_count(&self) -> usize {
        self.sample_count() / self.channels as usize
    }

    /// Returns the number of samples contained.
    pub fn sample_count(&self) -> usize {
        self.data.len() / self.format.size_in_bytes()
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }
}
