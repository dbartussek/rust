use crate::cmp::Ordering;
use crate::time::Duration;

use core::hash::{Hash, Hasher};

pub use self::inner::{Instant, SystemTime, UNIX_EPOCH};
use crate::convert::TryInto;

const NSEC_PER_SEC: u64 = 1_000_000_000;

#[derive(Copy, Clone)]
struct Timespec {
    t: libc::timespec,
}

impl Timespec {
    const fn zero() -> Timespec {
        Timespec { t: libc::timespec { tv_sec: 0, tv_nsec: 0 } }
    }

    fn sub_timespec(&self, other: &Timespec) -> Result<Duration, Duration> {
        if self >= other {
            // NOTE(eddyb) two aspects of this `if`-`else` are required for LLVM
            // to optimize it into a branchless form (see also #75545):
            //
            // 1. `self.t.tv_sec - other.t.tv_sec` shows up as a common expression
            //    in both branches, i.e. the `else` must have its `- 1`
            //    subtraction after the common one, not interleaved with it
            //    (it used to be `self.t.tv_sec - 1 - other.t.tv_sec`)
            //
            // 2. the `Duration::new` call (or any other additional complexity)
            //    is outside of the `if`-`else`, not duplicated in both branches
            //
            // Ideally this code could be rearranged such that it more
            // directly expresses the lower-cost behavior we want from it.
            let (secs, nsec) = if self.t.tv_nsec >= other.t.tv_nsec {
                ((self.t.tv_sec - other.t.tv_sec) as u64, (self.t.tv_nsec - other.t.tv_nsec) as u32)
            } else {
                (
                    (self.t.tv_sec - other.t.tv_sec - 1) as u64,
                    self.t.tv_nsec as u32 + (NSEC_PER_SEC as u32) - other.t.tv_nsec as u32,
                )
            };

            Ok(Duration::new(secs, nsec))
        } else {
            match other.sub_timespec(self) {
                Ok(d) => Err(d),
                Err(d) => Ok(d),
            }
        }
    }

    fn checked_add_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = other
            .as_secs()
            .try_into() // <- target type would be `libc::time_t`
            .ok()
            .and_then(|secs| self.t.tv_sec.checked_add(secs))?;

        // Nano calculations can't overflow because nanos are <1B which fit
        // in a u32.
        let mut nsec = other.subsec_nanos() + self.t.tv_nsec as u32;
        if nsec >= NSEC_PER_SEC as u32 {
            nsec -= NSEC_PER_SEC as u32;
            secs = secs.checked_add(1)?;
        }
        Some(Timespec { t: libc::timespec { tv_sec: secs, tv_nsec: nsec as _ } })
    }

    fn checked_sub_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = other
            .as_secs()
            .try_into() // <- target type would be `libc::time_t`
            .ok()
            .and_then(|secs| self.t.tv_sec.checked_sub(secs))?;

        // Similar to above, nanos can't overflow.
        let mut nsec = self.t.tv_nsec as i32 - other.subsec_nanos() as i32;
        if nsec < 0 {
            nsec += NSEC_PER_SEC as i32;
            secs = secs.checked_sub(1)?;
        }
        Some(Timespec { t: libc::timespec { tv_sec: secs, tv_nsec: nsec as _ } })
    }
}

impl PartialEq for Timespec {
    fn eq(&self, other: &Timespec) -> bool {
        self.t.tv_sec == other.t.tv_sec && self.t.tv_nsec == other.t.tv_nsec
    }
}

impl Eq for Timespec {}

impl PartialOrd for Timespec {
    fn partial_cmp(&self, other: &Timespec) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timespec {
    fn cmp(&self, other: &Timespec) -> Ordering {
        let me = (self.t.tv_sec, self.t.tv_nsec);
        let other = (other.t.tv_sec, other.t.tv_nsec);
        me.cmp(&other)
    }
}

impl Hash for Timespec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.tv_sec.hash(state);
        self.t.tv_nsec.hash(state);
    }
}

mod inner {
    use crate::fmt;
    use crate::sys::cvt;
    use crate::time::Duration;

    use super::Timespec;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Instant {
        t: Timespec,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SystemTime {
        t: Timespec,
    }

    pub const UNIX_EPOCH: SystemTime = SystemTime { t: Timespec::zero() };

    impl Instant {
        pub fn now() -> Instant {
            use super::NSEC_PER_SEC;

            let tick = unsafe { crate::sys::ctru::svcGetSystemTick() };
            let time = crate::sys_common::mul_div_u64(tick, 1_000_000_000, 268_111_856);

            let tv_sec = (time / NSEC_PER_SEC) as i32;
            let tv_nsec = (time % NSEC_PER_SEC) as i32;

            let t = Timespec { t: libc::timespec { tv_sec, tv_nsec } };

            Instant { t }
        }

        pub const fn zero() -> Instant {
            Instant { t: Timespec::zero() }
        }

        pub fn actually_monotonic() -> bool {
            (cfg!(target_os = "linux") && cfg!(target_arch = "x86_64"))
                || (cfg!(target_os = "linux") && cfg!(target_arch = "x86"))
                || cfg!(target_os = "fuchsia")
        }

        pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
            self.t.sub_timespec(&other.t).ok()
        }

        pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
            Some(Instant { t: self.t.checked_add_duration(other)? })
        }

        pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
            Some(Instant { t: self.t.checked_sub_duration(other)? })
        }
    }

    impl fmt::Debug for Instant {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Instant")
                .field("tv_sec", &self.t.t.tv_sec)
                .field("tv_nsec", &self.t.t.tv_nsec)
                .finish()
        }
    }

    impl SystemTime {
        pub fn now() -> SystemTime {
            let mut t = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            cvt(unsafe {
                libc::gettimeofday(&mut t, crate::ptr::null_mut())
            }).unwrap();
            SystemTime::from(t)
        }

        pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
            self.t.sub_timespec(&other.t)
        }

        pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
            Some(SystemTime { t: self.t.checked_add_duration(other)? })
        }

        pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
            Some(SystemTime { t: self.t.checked_sub_duration(other)? })
        }
    }

    impl From<libc::timeval> for SystemTime {
        fn from(t: libc::timeval) -> SystemTime {
            SystemTime::from(libc::timespec {
                tv_sec: t.tv_sec,
                tv_nsec: (t.tv_usec * 1000) as libc::c_long,
            })
        }
    }

    impl From<libc::timespec> for SystemTime {
        fn from(t: libc::timespec) -> SystemTime {
            SystemTime { t: Timespec { t } }
        }
    }

    impl fmt::Debug for SystemTime {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SystemTime")
                .field("tv_sec", &self.t.t.tv_sec)
                .field("tv_nsec", &self.t.t.tv_nsec)
                .finish()
        }
    }
}
