use std::cell::UnsafeCell;
use std::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};

// This is made repr(C) to enable some transmutation hacks for Decoders.
#[repr(C)]
pub struct SpinRwLock<T> {
    /// This either contains the number of readers using all of the bits above the first (0th bit),
    /// or the 0th bit will be set meaning that there is one writer with control of the lock.
    lock: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> SpinRwLock<T> {
    #[allow(dead_code)]
    pub fn new(data: T) -> SpinRwLock<T> {
        SpinRwLock {
            lock: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            if let Some(guard) = self.try_write() {
                return guard;
            } else {
                spin_loop_hint();
            }
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            if let Some(guard) = self.try_read() {
                return guard;
            } else {
                spin_loop_hint();
            }
        }
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        // Try to set the writer bit if there are no other readers and writers.
        if self.lock.compare_and_swap(0, 1, Ordering::Acquire) == 0 {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        // Try to just acquire the lock first by adding.
        self.lock.fetch_add(2, Ordering::Release);

        // If there was no writer at the time that the lock was acquired, then the lock was
        // acquired successfully.
        if (self.lock.load(Ordering::Acquire) & 1) == 0 {
            Some(RwLockReadGuard { lock: self })
        } else {
            // If there was a writer at the that that the lock was acquired, remove our lock.
            self.lock.fetch_sub(2, Ordering::Release);
            None
        }
    }

    fn release_write(&self) {
        // We use a sub here instead of a store so that we don't mess with other readers trying to
        // take the lock end up wrapping the value. This is also good because if a reader tries
        // to take the lock while the write lock is being released then there is a better chance
        // that the read lock will be successful.
        self.lock.fetch_sub(1, Ordering::Release);
    }

    fn release_read(&self) {
        self.lock.fetch_sub(2, Ordering::Release);
    }
}

pub struct RwLockReadGuard<'a, T> {
    lock: &'a SpinRwLock<T>,
}

impl<'a, T> std::ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.lock.data.get().as_ref().unwrap() }
    }
}

impl<'a, T> Drop for RwLockReadGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.release_read();
    }
}

pub struct RwLockWriteGuard<'a, T> {
    lock: &'a SpinRwLock<T>,
}

impl<'a, T> std::ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.lock.data.get().as_ref().unwrap() }
    }
}

impl<'a, T> std::ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.lock.data.get().as_mut().unwrap() }
    }
}

impl<'a, T> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.release_write();
    }
}
