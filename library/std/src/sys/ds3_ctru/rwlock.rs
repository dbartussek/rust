use super::mutex::Mutex;

/// TODO This is obviusly not a proper implementation, just a quick hacky one.
pub struct RWLock(Mutex);

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock(Mutex::new())
    }

    #[inline]
    pub unsafe fn read(&self) {
        self.write()
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        self.try_write()
    }

    #[inline]
    pub unsafe fn write(&self) {
        self.0.lock()
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        self.0.try_lock()
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        self.write_unlock()
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        self.0.unlock()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        self.0.destroy();
    }
}
