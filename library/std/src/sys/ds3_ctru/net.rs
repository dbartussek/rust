use crate::convert::TryFrom;
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::sys::{unsupported, Void};
use crate::time::Duration;
use crate::ops::Deref;
use crate::os::raw::*;

pub fn init() {
}

#[allow(nonstandard_style)]
pub type wrlen_t = usize;

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

pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    unsupported()
}
/// A variant of `cvt` for `getaddrinfo` which return 0 for a success.
pub fn cvt_gai(err: c_int) -> io::Result<()> {
    unsupported()
}

/// Just to provide the same interface as sys/unix/net.rs
pub fn cvt_r<T, F>(mut f: F) -> io::Result<T>
    where
        T: IsMinusOne,
        F: FnMut() -> T,
{
    cvt(f())
}

pub struct SocketInner(Void);


impl Deref for SocketInner {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        match self.0 {}
    }
}

impl fmt::Debug for SocketInner {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

pub struct Socket{
    inner: SocketInner
}

impl Socket {
    pub fn new(addr: &SocketAddr, flags: c_int) -> io::Result<Self> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<Self> {
        match self.inner.0 {}
    }

    pub fn as_inner(&self) -> SocketInner {
        match self.inner.0 {}
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> ! {
        match self.inner.0 {}
    }

    pub fn take_error(&self) -> ! {
        match self.inner.0 {}
    }

    pub fn peek(&self, buf: &mut[u8]) -> ! {
        match self.inner.0 {}
    }

    pub fn timeout(&self, kind: c_int) -> io::Result<Option<Duration>> {
        match self.inner.0 {}
    }
    pub fn set_timeout(&self, dur: Option<Duration>, kind: c_int) -> io::Result<()> {
        match self.inner.0 {}
    }

    pub fn read(&self, buf: &mut [u8]) -> ! {
        match self.inner.0 {}
    }

    pub fn peek_from(&self, buf: &mut[u8]) -> ! {
        match self.inner.0 {}
    }
    pub fn recv_from(&self, buf: &mut[u8]) -> ! {
        match self.inner.0 {}
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        match self.inner.0 {}
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        match self.inner.0 {}
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        match self.inner.0 {}
    }

    pub fn accept(&self, storage: *mut SocketAddr, len: *mut c_uint) -> io::Result<Socket> {
        match self.inner.0 {}
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write_vectored(&self, bufs: &[IoSlice]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        unsupported()
    }

    pub fn connect_timeout(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<()> {
        unsupported()
    }
}

pub struct TcpStream(Void);

impl TcpStream {
    pub fn connect(_: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        match self.0 {}
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        match self.0 {}
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        match self.0 {}
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        match self.0 {}
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn read(&self, _: &mut [u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn is_read_vectored(&self) -> bool {
        match self.0 {}
    }

    pub fn write(&self, _: &[u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn is_write_vectored(&self) -> bool {
        match self.0 {}
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        match self.0 {}
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        match self.0 {}
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn ttl(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        match self.0 {}
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

pub struct TcpListener(Void);

impl TcpListener {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        match self.0 {}
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        match self.0 {}
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn ttl(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        match self.0 {}
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

pub struct UdpSocket(Void);

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        match self.0 {}
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        match self.0 {}
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        match self.0 {}
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        match self.0 {}
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        match self.0 {}
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        match self.0 {}
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        match self.0 {}
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        match self.0 {}
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        match self.0 {}
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        match self.0 {}
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        match self.0 {}
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        match self.0 {}
    }

    pub fn ttl(&self) -> io::Result<u32> {
        match self.0 {}
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        match self.0 {}
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        match self.0 {}
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        match self.0 {}
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        match self.0 {}
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

pub struct LookupHost(Void);

impl LookupHost {
    pub fn port(&self) -> u16 {
        match self.0 {}
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        match self.0 {}
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: &str) -> io::Result<LookupHost> {
        unsupported()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: (&'a str, u16)) -> io::Result<LookupHost> {
        unsupported()
    }
}

pub use libc as netc;
