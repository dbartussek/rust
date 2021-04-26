use crate::io as std_io;
use crate::os::raw::*;

pub mod memchr {
    pub use core::slice::memchr::{memchr, memrchr};
}

pub use crate::sys_common::os_str_bytes as os_str;

// This is not necessarily correct. May want to consider making it part of the
// spec definition?
use crate::os::raw::c_char;

#[cfg(not(test))]
pub fn init() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::new_const(
        std_io::ErrorKind::Unsupported,
        &"operation not supported on this platform",
    )
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    std_io::ErrorKind::Other
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut s: *const c_char) -> usize {
    // SAFETY: The caller must guarantee `s` points to a valid 0-terminated string.
    unsafe {
        let mut n = 0;
        while *s != 0 {
            n += 1;
            s = s.offset(1);
        }
        n
    }
}


#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
    if t.is_minus_one() { Err(crate::io::Error::last_os_error()) } else { Ok(t) }
}

pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
    where
        T: IsMinusOne,
        F: FnMut() -> T,
{
    loop {
        match cvt(f()) {
            Err(ref e) if e.kind() == crate::io::ErrorKind::Interrupted => {}
            other => return other,
        }
    }
}

pub fn cvt_nz(error: libc::c_int) -> crate::io::Result<()> {
    if error == 0 { Ok(()) } else { Err(crate::io::Error::from_raw_os_error(error)) }
}

/// A variant of `cvt` for `getaddrinfo` which return 0 for a success.
pub fn cvt_gai(err: c_int) -> crate::io::Result<()> {
    unsupported()
}
