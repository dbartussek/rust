use super::ctru;
use crate::cell::UnsafeCell;
use crate::mem::MaybeUninit;

pub struct Mutex(UnsafeCell<ctru::LightLock>);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

pub type MovableMutex = Box<Mutex>;

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex(UnsafeCell::new(1))
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        ctru::LightLock_Init(self.0.get())
    }

    #[inline]
    pub unsafe fn lock(&self) {
        ctru::LightLock_Lock(self.0.get())
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        ctru::LightLock_Unlock(self.0.get())
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        ctru::LightLock_TryLock(self.0.get()) == 0
    }

    #[inline]
    pub unsafe fn destroy(&self) {}
}

pub struct ReentrantMutex(UnsafeCell<ctru::RecursiveLock>);

impl ReentrantMutex {
    pub const unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex(UnsafeCell::new(ctru::RecursiveLock{
            lock: 1,
            thread_tag: 0,
            counter: 0,
        }))
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        ctru::RecursiveLock_Init(self.0.get())
    }

    #[inline]
    pub unsafe fn lock(&self) {
        ctru::RecursiveLock_Lock(self.0.get())
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        ctru::RecursiveLock_Unlock(self.0.get())
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        ctru::RecursiveLock_TryLock(self.0.get()) == 0
    }

    #[inline]
    pub unsafe fn destroy(&self) {}
}
