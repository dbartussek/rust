use super::{unsupported, ctru};
use crate::ffi::CStr;
use crate::io;
use crate::time::Duration;
use crate::os::raw::c_void;
use crate::mem::transmute;

pub struct Thread(ctru::Thread);

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        unsafe extern "C" fn thread_runner(arg: *mut c_void) {
            let p: Box<Box<dyn FnOnce()>> = Box::from_raw(transmute(arg));
            p()
        }

        let thread = ctru::threadCreate(
            Some(thread_runner),
            transmute(Box::into_raw(Box::new(p))),
            stack,
            0x30,
            -1,
            false,
        );

        if thread.is_null() {
            // TODO handle thread failure
        }

        Ok(Thread(thread))
    }

    pub fn yield_now() {
        // do nothing
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(_dur: Duration) {
        panic!("can't sleep");
    }

    pub fn join(self) {
        unsafe {
            ctru::threadJoin(self.0, crate::u64::MAX);
        }
    }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
