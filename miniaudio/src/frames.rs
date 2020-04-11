use crate::base::*;
use crate::conversion::conversion_fn_for;

pub struct Frames<'s> {
    data: &'s [u8],
    format: Format,
    channels: u32,
}

impl<'s> Frames<'s> {
    #[inline]
    pub fn wrap(data: &'s [u8], format: Format, channels: u32) -> Frames<'s> {
        Frames {
            data,
            format,
            channels,
        }
    }

    /// Convert this frames samples into another format, placing the new converted
    /// frames into `dest`.
    #[inline]
    pub fn convert(&self, dest: &mut FramesMut, dither_mode: DitherMode) {
        assert!(
            self.frame_count() == dest.frame_count(),
            "frame conversion with different frame counts (input: {}, output: {})",
            self.frame_count(),
            dest.frame_count()
        );

        let convert_fn = conversion_fn_for(self.format, dest.format);
        unsafe {
            convert_fn(
                dest.as_mut_ptr() as *mut _,
                self.as_ptr() as *const _,
                self.frame_count() as u64,
                dither_mode as _,
            );
        }
    }

    pub(crate) fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub fn as_samples<S: Sample>(&self) -> &[S] {
        assert!(
            self.format() == S::format(),
            "format mismatch (frames: {:?}, requested: {:?})",
            self.format,
            S::format()
        );

        let len = self.sample_count();
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<S>(), len) }
    }

    #[inline]
    pub fn frames<'t, S: 'static + Sample>(&'t self) -> impl 't + Iterator<Item = &[S]> {
        FramesIter {
            samples: self.as_samples(),
            channels: self.channels,
            offset: 0,
        }
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of frames contained.
    #[inline]
    pub fn frame_count(&self) -> usize {
        self.sample_count() / self.channels as usize
    }

    /// Returns the number of samples contained.
    #[inline]
    pub fn sample_count(&self) -> usize {
        self.data.len() / self.format.size_in_bytes()
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    #[inline]
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
    #[inline]
    pub fn wrap(data: &'s mut [u8], format: Format, channels: u32) -> FramesMut<'s> {
        FramesMut {
            data,
            format,
            channels,
        }
    }

    pub(crate) fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_ptr() as *mut u8
    }

    /// Convert this frames samples into another format, placing the new converted
    /// frames into `dest`.
    #[inline]
    pub fn convert(&self, dest: &mut FramesMut, dither_mode: DitherMode) {
        assert!(
            self.frame_count() == dest.frame_count(),
            "frame conversion with different frame counts (input: {}, output: {})",
            self.frame_count(),
            dest.frame_count()
        );

        let convert_fn = conversion_fn_for(self.format, dest.format);
        unsafe {
            convert_fn(
                dest.as_mut_ptr() as *mut _,
                self.as_ptr() as *const _,
                self.frame_count() as u64,
                dither_mode as _,
            );
        }
    }

    #[inline]
    pub fn as_samples<S: Sample>(&self) -> &[S] {
        assert!(
            self.format() == S::format(),
            "format mismatch (frames: {:?}, requested: {:?})",
            self.format,
            S::format()
        );

        let len = self.sample_count();
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<S>(), len) }
    }

    #[inline]
    pub fn as_samples_mut<S: Sample>(&mut self) -> &mut [S] {
        assert!(
            self.format() == S::format(),
            "format mismatch (frames: {:?}, requested: {:?})",
            self.format,
            S::format()
        );

        let len = self.sample_count();
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr().cast::<S>(), len) }
    }

    #[inline]
    pub fn frames<'t, S: 'static + Sample>(&'t self) -> impl 't + Iterator<Item = &[S]> {
        FramesIter {
            samples: self.as_samples(),
            channels: self.channels,
            offset: 0,
        }
    }

    #[inline]
    pub fn frames_mut<'t, S: 'static + Sample>(
        &'t mut self,
    ) -> impl 't + Iterator<Item = &mut [S]> {
        let channels = self.channels;
        let samples = self.as_samples_mut();
        let samples_len = samples.len();

        FramesIterMut {
            samples_ptr: samples.as_mut_ptr(),
            len: samples_len,
            channels,
            offset: 0,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.data
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of frames contained.
    #[inline]
    pub fn frame_count(&self) -> usize {
        self.sample_count() / self.channels as usize
    }

    /// Returns the number of samples contained.
    #[inline]
    pub fn sample_count(&self) -> usize {
        self.data.len() / self.format.size_in_bytes()
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.channels
    }
}

pub struct FramesIter<'s, S: Sample> {
    samples: &'s [S],
    channels: u32,
    offset: usize,
}

impl<'s, S: Sample> Iterator for FramesIter<'s, S> {
    type Item = &'s [S];

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= (self.samples.len() - self.channels as usize) {
            None
        } else {
            let ret = Some(&self.samples[self.offset..(self.offset + self.channels as usize)]);
            self.offset += self.channels as usize;
            ret
        }
    }
}

pub struct FramesIterMut<'s, S: Sample> {
    samples_ptr: *mut S,
    len: usize,
    channels: u32,
    offset: usize,
    phantom: std::marker::PhantomData<&'s S>,
}

impl<'s, S: Sample> Iterator for FramesIterMut<'s, S> {
    type Item = &'s mut [S];

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= (self.len - self.channels as usize) {
            None
        } else {
            // FIXME The compiler doesn't like me doing the same thing that I do in FramesIter in here
            // using mut and gives me some cryptic error, so I'm just using pointers for now.
            let ret = Some(unsafe {
                std::slice::from_raw_parts_mut(
                    self.samples_ptr.add(self.offset),
                    self.channels as usize,
                )
            });
            self.offset += self.channels as usize;
            ret
        }
    }
}

pub trait Sample {
    /// Returns the format of this sample.
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
