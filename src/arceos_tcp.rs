use std::io as io;
use std::os::arceos::api::net as api;
use std::net::SocketAddr;
use std::os::arceos::api::AxResult;
pub use std::os::arceos::net::{AxTcpSocketHandle, IntoRawTcpSocket};
use std::sync::Arc;

#[inline]
fn cvt<T>(t: AxResult<T>) -> std::io::Result<T> {
    match t {
        Ok(t) => Ok(t),
        Err(e) => Err(std::io::Error::from_raw_os_error(e.code())),
    }
}

pub fn new_tcp_socket_handle() -> io::Result<AxTcpSocketHandle> {
    let socket = api::ax_tcp_socket();
    //cvt(api::ax_tcp_set_nonblocking(&socket, true)).unwrap();
    Ok(socket)
}

pub fn bind(socket: &Arc<AxTcpSocketHandle>, addr: SocketAddr) -> io::Result<()> {
    cvt(api::ax_tcp_bind(socket.as_ref(), addr))
}

pub fn connect(socket: &Arc<AxTcpSocketHandle>, addr: SocketAddr) -> io::Result<()> {
    match cvt(api::ax_tcp_connect(socket.as_ref(), addr)) {
        Err(err) if err.kind() != io::ErrorKind::WouldBlock => Err(err),
        _ => Ok(()),
    }
}

pub fn listen(socket: &Arc<AxTcpSocketHandle>, backlog: u32) -> io::Result<()> {
    cvt(api::ax_tcp_listen(socket.as_ref(), backlog as usize))
}

pub fn accept(socket: &Arc<AxTcpSocketHandle>) -> io::Result<(Arc<AxTcpSocketHandle>, SocketAddr)> {
    let (handle, addr) = cvt(api::ax_tcp_accept(socket.as_ref()))?;
    //cvt(api::ax_tcp_set_nonblocking(&handle, true))?;
    let socket = unsafe { Arc::new(handle) };
    Ok((socket, addr))
}

pub fn send(socket: &Arc<AxTcpSocketHandle>, buf: &[u8]) -> io::Result<usize> {
    cvt(api::ax_tcp_send(socket, buf))
}

pub fn recv(socket: &Arc<AxTcpSocketHandle>, buf: &mut [u8]) -> io::Result<usize> {
    cvt(api::ax_tcp_recv(socket, buf))
}
