//! Miniaudio supports lock free (single producer, single consumer) ring buffers which are exposed
//! via the `RingBuffer` and `PCMRingBuffer` APIs. The `RingBuffer` API operats on bytes, whereas
//! the `PCMRingBuffer` operates on PCM frames. They are otherwise identical as `PCMRingBuffer` is
//! just a wrapper around `RingBuffer`.

use crate::base::{from_bool32, Error};
use miniaudio_sys as sys;
use std::os::raw::c_void;
use std::ptr::NonNull;

#[repr(transparent)]
#[derive(Debug)]
pub struct RingBuffer<T: Sized> {
    inner: sys::ma_rb,
    _buffer_type: std::marker::PhantomData<T>,
}

impl<T: Sized> RingBuffer<T> {
    pub fn new(count: usize) -> Result<RingBuffer<T>, Error> {
        let stride_in_bytes = std::mem::size_of::<T>();
        let size_in_bytes = count * stride_in_bytes;

        unsafe { Self::new_raw(size_in_bytes, count, stride_in_bytes, None) }
    }

    pub fn new_preallocated(preallocated: Box<[T]>) -> Result<RingBuffer<T>, Error> {
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
            if let &Err(_) = &result {
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

    pub fn read<F>(&self, count_requested: usize, f: F) -> Result<usize, Error>
    where
        F: FnOnce(&[T]),
    {
        let mut bytes = count_requested * std::mem::size_of::<T>();
        let mut buf_ptr: *mut c_void = std::ptr::null_mut();
        let acquire_result = unsafe {
            sys::ma_rb_acquire_read(&self.inner as *const _ as *mut _, &mut bytes, &mut buf_ptr)
        };
        debug_assert!(bytes % std::mem::size_of::<T>() == 0);
        let count = bytes / std::mem::size_of::<T>();

        if count == 0 || buf_ptr.is_null() {
            f(&[]);
            return Ok(0);
        }

        let items = map_result!(acquire_result, unsafe {
            std::slice::from_raw_parts(buf_ptr.cast::<T>(), count)
        })?;

        f(items);

        map_result!(
            unsafe { sys::ma_rb_commit_read(&self.inner as *const _ as *mut _, bytes, buf_ptr) },
            count
        )
    }

    pub fn write<F>(&self, count_requested: usize, f: F) -> Result<usize, Error>
    where
        F: FnOnce(&mut [T]),
    {
        let mut bytes = count_requested * std::mem::size_of::<T>();
        let mut buf_ptr: *mut c_void = std::ptr::null_mut();
        let acquire_result = unsafe {
            sys::ma_rb_acquire_write(&self.inner as *const _ as *mut _, &mut bytes, &mut buf_ptr)
        };
        debug_assert!(bytes % std::mem::size_of::<T>() == 0);
        let count = bytes / std::mem::size_of::<T>();

        if count == 0 || buf_ptr.is_null() {
            f(&mut []);
            return Ok(0);
        }

        let items = map_result!(acquire_result, unsafe {
            std::slice::from_raw_parts_mut(buf_ptr.cast::<T>(), count)
        })?;

        f(items);

        map_result!(
            unsafe { sys::ma_rb_commit_write(&self.inner as *const _ as *mut _, bytes, buf_ptr) },
            count
        )
    }

    /// Returns the distance between the write pointer and the read pointer. Should never be
    /// negative for a correct program. Will return the number of items that can be read before the
    /// read pointer hits the write pointer.
    #[inline]
    pub fn pointer_distance(&self) -> usize {
        let byte_distance =
            unsafe { sys::ma_rb_pointer_distance(&self.inner as *const _ as *mut _) as usize };
        debug_assert!(byte_distance % std::mem::size_of::<T>() == 0);
        byte_distance / std::mem::size_of::<T>()
    }

    #[inline]
    pub fn available_read(&self) -> usize {
        let bytes_available =
            unsafe { sys::ma_rb_available_read(&self.inner as *const _ as *mut _) as usize };
        debug_assert!(bytes_available % std::mem::size_of::<T>() == 0);
        bytes_available / std::mem::size_of::<T>()
    }

    #[inline]
    pub fn available_write(&self) -> usize {
        let bytes_available =
            unsafe { sys::ma_rb_available_write(&self.inner as *const _ as *mut _) as usize };
        debug_assert!(bytes_available % std::mem::size_of::<T>() == 0);
        bytes_available / std::mem::size_of::<T>()
    }

    #[inline]
    pub fn subbuffer_size(&self) -> usize {
        unsafe { sys::ma_rb_get_subbuffer_size(&self.inner as *const _ as *mut _) }
    }

    #[inline]
    pub fn subbuffer_stride(&self) -> usize {
        unsafe { sys::ma_rb_get_subbuffer_stride(&self.inner as *const _ as *mut _) }
    }

    // FIXME document this (???)
    #[inline]
    pub fn subbuffer_offset(&self, index: usize) -> usize {
        unsafe { sys::ma_rb_get_subbuffer_offset(&self.inner as *const _ as *mut _, index) }
    }

    // FIXME implement the seek_read and seek_write functions when I figure out what those are for
    // really.
}

unsafe impl<T: Send + Sized> Send for RingBuffer<T> {}
unsafe impl<T: Sync + Sized> Sync for RingBuffer<T> {}

impl<T> Drop for RingBuffer<T> {
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
                std::mem::forget(preallocated_slice); // we don't care about that :P
            }
        };
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct PCMRingBuffer(sys::ma_pcm_rb);
