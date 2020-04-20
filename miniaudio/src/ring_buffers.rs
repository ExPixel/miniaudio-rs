//! Miniaudio supports lock free (single producer, single consumer) ring buffers which are exposed
//! via the `RingBuffer` and `PCMRingBuffer` APIs. The `RingBuffer` API operats on bytes, whereas
//! the `PCMRingBuffer` operates on PCM frames. They are otherwise identical as `PCMRingBuffer` is
//! just a wrapper around `RingBuffer`.

use crate::base::{from_bool32, Error};
use miniaudio_sys as sys;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct RingBuffer<T: Clone> {
    inner: sys::ma_rb,
    _buffer_type: std::marker::PhantomData<T>,
}

impl<T: Clone> RingBuffer<T> {
    pub(crate) fn split(self) -> (RingBufferSend<T>, RingBufferRecv<T>) {
        let wrapped = Arc::new(self);
        let recv = RingBufferRecv {
            inner: Arc::clone(&wrapped),
        };
        let send = RingBufferSend { inner: wrapped };
        (send, recv)
    }

    pub(crate) fn create_pair(
        count: usize,
    ) -> Result<(RingBufferSend<T>, RingBufferRecv<T>), Error> {
        RingBuffer::new(count).map(Self::split)
    }

    pub(crate) fn create_pair_preallocated(
        preallocated: Box<[T]>,
    ) -> Result<(RingBufferSend<T>, RingBufferRecv<T>), Error> {
        RingBuffer::new_preallocated(preallocated).map(Self::split)
    }

    pub(crate) fn new(count: usize) -> Result<RingBuffer<T>, Error> {
        let stride_in_bytes = std::mem::size_of::<T>();
        let size_in_bytes = count * stride_in_bytes;

        unsafe { Self::new_raw(size_in_bytes, count, stride_in_bytes, None) }
    }

    pub(crate) fn new_preallocated(preallocated: Box<[T]>) -> Result<RingBuffer<T>, Error> {
        let count = preallocated.len();
        let stride_in_bytes = std::mem::size_of::<T>();
        let size_in_bytes = count * stride_in_bytes;
        let preallocated_ptr_slice = Box::into_raw(preallocated);

        unsafe {
            let preallocated_ptr = (*preallocated_ptr_slice).as_mut_ptr();
            let result = Self::new_raw(
                size_in_bytes,
                count,
                stride_in_bytes,
                NonNull::new(preallocated_ptr).map(NonNull::cast),
            );

            // If an error occurred, we need to drop the box.
            if result.is_err() {
                drop(Box::from_raw(preallocated_ptr_slice));
            }

            result
        }
    }

    unsafe fn new_raw(
        subbuffer_size_in_bytes: usize,
        subbuffer_count: usize,
        subbuffer_stride_in_bytes: usize,
        preallocated_buffer: Option<NonNull<()>>,
    ) -> Result<RingBuffer<T>, Error> {
        let mut ring_buffer = std::mem::MaybeUninit::<sys::ma_rb>::uninit();

        let result = sys::ma_rb_init_ex(
            subbuffer_size_in_bytes,
            subbuffer_count,
            subbuffer_stride_in_bytes,
            preallocated_buffer
                .map(|p| p.cast().as_ptr())
                .unwrap_or(std::ptr::null_mut()),
            std::ptr::null(),
            ring_buffer.as_mut_ptr(),
        );

        map_result!(
            result,
            RingBuffer {
                inner: ring_buffer.assume_init(),
                _buffer_type: std::marker::PhantomData,
            }
        )
    }

    /// Used to retrieve a section of the ring buffer for reading. You specify the number of items
    /// you would like to read and a slice with the number of requested items (or less if the
    /// buffer needs to wrap), will be passed to the given closure.
    pub(crate) fn read<F>(&self, count_requested: usize, f: F) -> usize
    where
        F: FnOnce(&[T]),
    {
        let mut bytes = count_requested * std::mem::size_of::<T>();
        let mut buf_ptr: *mut c_void = std::ptr::null_mut();
        let acquire_result = unsafe {
            sys::ma_rb_acquire_read(&self.inner as *const _ as *mut _, &mut bytes, &mut buf_ptr)
        };

        // This shouldn't fail because our arguments are valid, but we debug assert just to be sure.
        debug_assert!(acquire_result == 0);
        debug_assert!(bytes % std::mem::size_of::<T>() == 0);

        let count = bytes / std::mem::size_of::<T>();

        if count == 0 || buf_ptr.is_null() {
            f(&[]);
            return 0;
        }

        let items = unsafe { std::slice::from_raw_parts(buf_ptr.cast::<T>(), count) };

        f(items);

        let commit_result =
            unsafe { sys::ma_rb_commit_read(&self.inner as *const _ as *mut _, bytes, buf_ptr) };

        // This shouldn't fail because our arguments are valid, but we debug assert just to be sure.
        debug_assert!(commit_result == 0);

        count
    }

    /// Used to retrieve a section of the ring buffer for writing. You specify the number of items
    /// you would like to write to and a slice with the number of requested items (or less if the
    /// buffer needs to wrap), will be passed to the given closure.
    pub(crate) fn write<F>(&self, count_requested: usize, f: F) -> usize
    where
        F: FnOnce(&mut [T]),
    {
        let mut bytes = count_requested * std::mem::size_of::<T>();
        let mut buf_ptr: *mut c_void = std::ptr::null_mut();
        let acquire_result = unsafe {
            sys::ma_rb_acquire_write(&self.inner as *const _ as *mut _, &mut bytes, &mut buf_ptr)
        };

        // This shouldn't fail because our arguments are valid, but we debug assert just to be sure.
        debug_assert!(acquire_result == 0);
        debug_assert!(bytes % std::mem::size_of::<T>() == 0);

        let count = bytes / std::mem::size_of::<T>();

        if count == 0 || buf_ptr.is_null() {
            f(&mut []);
            return 0;
        }

        let items = unsafe { std::slice::from_raw_parts_mut(buf_ptr.cast::<T>(), count) };

        f(items);

        let commit_result =
            unsafe { sys::ma_rb_commit_write(&self.inner as *const _ as *mut _, bytes, buf_ptr) };

        // This shouldn't fail because our arguments are valid, but we debug assert just to be sure.
        debug_assert!(commit_result == 0);

        count
    }

    // FIXME find out what to do with this and remove allow(dead_code).
    /// Returns the distance between the write pointer and the read pointer. Should never be
    /// negative for a correct program. Will return the number of items that can be read before the
    /// read pointer hits the write pointer.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn pointer_distance(&self) -> usize {
        let byte_distance =
            unsafe { sys::ma_rb_pointer_distance(&self.inner as *const _ as *mut _) as usize };
        debug_assert!(byte_distance % std::mem::size_of::<T>() == 0);
        byte_distance / std::mem::size_of::<T>()
    }

    #[inline]
    pub(crate) fn available_read(&self) -> usize {
        let bytes_available =
            unsafe { sys::ma_rb_available_read(&self.inner as *const _ as *mut _) as usize };
        debug_assert!(bytes_available % std::mem::size_of::<T>() == 0);
        bytes_available / std::mem::size_of::<T>()
    }

    #[inline]
    pub(crate) fn available_write(&self) -> usize {
        let bytes_available =
            unsafe { sys::ma_rb_available_write(&self.inner as *const _ as *mut _) as usize };
        debug_assert!(bytes_available % std::mem::size_of::<T>() == 0);
        bytes_available / std::mem::size_of::<T>()
    }

    // FIXME find out what to do with this and remove allow(dead_code).
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn subbuffer_size(&self) -> usize {
        unsafe { sys::ma_rb_get_subbuffer_size(&self.inner as *const _ as *mut _) }
    }

    // FIXME find out what to do with this and remove allow(dead_code).
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn subbuffer_stride(&self) -> usize {
        unsafe { sys::ma_rb_get_subbuffer_stride(&self.inner as *const _ as *mut _) }
    }

    // FIXME document this (???)
    // FIXME find out what to do with this and remove allow(dead_code).
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn subbuffer_offset(&self, index: usize) -> usize {
        unsafe { sys::ma_rb_get_subbuffer_offset(&self.inner as *const _ as *mut _, index) }
    }

    // FIXME implement the seek_read and seek_write functions when I figure out what those are for
    // really.
}

unsafe impl<T: Send + Sized + Clone> Send for RingBuffer<T> {}
unsafe impl<T: Send + Sized + Clone> Sync for RingBuffer<T> {}

pub struct RingBufferSend<T: Clone> {
    inner: Arc<RingBuffer<T>>,
}

impl<T: Clone> RingBufferSend<T> {
    /// Write a buffer of items into the ring buffer, returning the number of items that were
    /// successfully written.
    pub fn write(&mut self, src: &[T]) -> usize {
        self.inner.write(src.len(), |dest| {
            dest.clone_from_slice(&src[0..dest.len()]);
        })
    }

    /// Returns the number of items that are available for writing.
    pub fn available(&mut self) -> usize {
        self.inner.available_write()
    }
}

pub struct RingBufferRecv<T: Clone> {
    inner: Arc<RingBuffer<T>>,
}

impl<T: Clone> RingBufferRecv<T> {
    pub fn read(&mut self, dest: &mut [T]) -> usize {
        self.inner.read(dest.len(), |src| {
            (&mut dest[0..src.len()]).clone_from_slice(src);
        })
    }

    /// Returns the number of items that are available for reading.
    pub fn available(&mut self) -> usize {
        self.inner.available_read()
    }
}

impl<T: Clone> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            let buffer_ptr = self.inner.pBuffer;
            let count = self.inner.subbufferCount;
            let owns_buffer = from_bool32(self.inner.ownsBuffer());

            sys::ma_rb_uninit(&mut self.inner);

            // If the buffer was not created by miniaudio we drop it from Rust.
            if !owns_buffer && !buffer_ptr.is_null() {
                let preallocated_slice = std::slice::from_raw_parts_mut(buffer_ptr, count as usize);
                let _preallocated_box = Box::from_raw(preallocated_slice.as_mut_ptr());
            }
        };
    }
}

/// Create a sender/receiver pair for a single producer single consumer ring buffer.
pub fn ring_buffer<T: Clone + Send>(
    max_item_count: usize,
) -> Result<(RingBufferSend<T>, RingBufferRecv<T>), Error> {
    RingBuffer::create_pair(max_item_count)
}

/// Create a sender/receiver pair for a single producer single consumer ring buffer using
/// a preallocated buffer for items.
pub fn ring_buffer_preallocated<T: Clone + Send>(
    preallocated: Box<[T]>,
) -> Result<(RingBufferSend<T>, RingBufferRecv<T>), Error> {
    RingBuffer::create_pair_preallocated(preallocated)
}

// FIXME remove this since it's probably useless (in Rust anyway).
#[deprecated = "Instead of using this, just use RingBuffer with the frame type as the type parameter."]
#[repr(transparent)]
#[derive(Debug)]
pub struct PCMRingBuffer(sys::ma_pcm_rb);
