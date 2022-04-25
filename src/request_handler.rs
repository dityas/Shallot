use std::fmt::Debug;
use std::io::ErrorKind::WouldBlock;
use std::io::Read;
use std::io::Write;

use std::net::Shutdown;
use std::net::TcpStream;

use httparse::{Request, EMPTY_HEADER};

use crate::logging;
use crate::proxy_listener::get_target_stream;
use crate::proxy_listener::ProxyError;
use crate::proxy_listener::Result;

/// HTTP responses from the proxy server
/// HTTP response for 200 OK
const HTTP_OK: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();

/// Parse request into a well defined request type
/// For now, the proxy only supports GET and CONNECT requests
#[derive(Debug)]
pub enum ReqType {
    CONNECT(String),
    GET(String),
}

impl std::fmt::Display for ReqType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReqType::CONNECT(a) => write!(f, "ReqType::CONNECT({})", a),
            ReqType::GET(a) => write!(f, "ReqType::GET({})", a),
        }
    }
}

/// Determine request type. Get the raw request and parse it into
/// a ReqType.
fn determine_request(buf: &[u8]) -> Result<ReqType> {
    let len = buf.len();
    let mut headers = [EMPTY_HEADER; 4096];
    let mut req = Request::new(&mut headers);
    let res = req
        .parse(&buf)
        .map_err(|_| ProxyError::Parse("While parsing request".to_owned()))?;

    if res.is_partial() {
        Err(ProxyError::Parse("Incomplete request".to_owned()))
    } else {
        match req.method {
            Some("CONNECT") => match req.path {
                Some(a) => Ok(ReqType::CONNECT(a.to_string())),
                None => Err(ProxyError::Parse("Could not parse path".to_owned())),
            },

            Some("GET") => match req.path {
                Some(p) => Ok(ReqType::GET(p.to_string())),
                None => Err(ProxyError::Parse("Could not parse path".to_owned())),
            },

            Some(p) => Err(ProxyError::Parse(format!("Could not parse {}", p))),
            None => Err(ProxyError::Parse("Could not parse request".to_owned())),
        }
    }
}

fn get_req_type(stream: &mut TcpStream) -> Result<ReqType> {
    let mut buf = [0u8; 4096];
    let read_bytes = read_from_tcpstream(stream, &mut buf)?;

    determine_request(&mut buf[0..read_bytes])
}

/// Wrappers to read from and write to asyn TcpStream
/// Wrapper to write to a stream
fn write_to_tcpstream(stream: &mut TcpStream, buf: &[u8]) -> Result<usize> {
    match stream.write_all(buf) {
        Ok(()) => match stream.flush() {
            Ok(_) => Ok(buf.len()),
            Err(e) => Err(ProxyError::IO(format!("While flushing {:?}", e))),
        },
        Err(ref e) if e.kind() == WouldBlock => Err(ProxyError::IOBlocked),
        Err(e) => Err(ProxyError::IO(format!("While writing {:?}", e))),
    }
}

/// Wrapper to read from a stream
fn read_from_tcpstream(stream: &mut TcpStream, buf: &mut [u8]) -> Result<usize> {
    match stream.read(buf) {
        Ok(0) => Err(ProxyError::StreamClosed),
        Ok(n) => Ok(n),

        Err(ref e) if e.kind() == WouldBlock => Err(ProxyError::IOBlocked),
        Err(e) => Err(ProxyError::IO(format!("While reading {:?}", e))),
    }
}

fn tunnel_bytes(s_stream: &mut TcpStream, t_stream: &mut TcpStream) -> usize {
    let mut bytes_transferred = 0usize;

    let mut source_to_target_buf = [0u8; 4096];
    let mut target_to_source_buf = [0u8; 4096];

    let mut source_bytes = 0usize;
    let mut target_bytes = 0usize;

    s_stream.set_nonblocking(true);
    t_stream.set_nonblocking(true);

    // Tunnel request
    loop {
        // Read from source into buf
        if source_bytes == 0 {
            match read_from_tcpstream(s_stream, &mut source_to_target_buf) {
                Ok(0) => {
                    t_stream.shutdown(Shutdown::Both);
                    break;
                }
                Ok(n) => {
                    //logging::event_log(
                    //    format!(
                    //        "{} bytes read from {}",
                    //        n,
                    //        s_stream.local_addr().unwrap().ip()
                    //    )
                    //    .as_str(),
                    //);

                    source_bytes = n;
                }
                Err(ProxyError::IOBlocked) => {}
                Err(e) => {
                    t_stream.shutdown(Shutdown::Both);
                    break;
                }
            };
        }

        if source_bytes > 0 {
            match write_to_tcpstream(t_stream, &mut source_to_target_buf[0..source_bytes]) {
                Ok(0) => {
                    s_stream.shutdown(Shutdown::Both);
                    break;
                }
                Ok(n) => {
                    //logging::event_log(
                    //    format!(
                    //        "{} bytes written to {}",
                    //        n,
                    //        t_stream.peer_addr().unwrap().ip()
                    //    )
                    //    .as_str(),
                    //);
                    bytes_transferred += source_bytes;
                    source_bytes = 0;
                }
                Err(ProxyError::IOBlocked) => {}
                Err(e) => {
                    s_stream.shutdown(Shutdown::Both);
                    break;
                }
            };
        }

        // Read from target
        if target_bytes == 0 {
            match read_from_tcpstream(t_stream, &mut target_to_source_buf) {
                Ok(0) => {
                    s_stream.shutdown(Shutdown::Both);
                    break;
                }
                Ok(n) => {
                    target_bytes = n;
                    //logging::event_log(
                    //    format!(
                    //        "{} bytes read from {}",
                    //        n,
                    //        t_stream.peer_addr().unwrap().ip()
                    //    )
                    //    .as_str(),
                    //);
                }
                Err(ProxyError::IOBlocked) => {}
                Err(e) => {
                    s_stream.shutdown(Shutdown::Both);
                    break;
                }
            };
        }

        if target_bytes > 0 {
            match write_to_tcpstream(s_stream, &mut target_to_source_buf[0..target_bytes]) {
                Ok(0) => {
                    t_stream.shutdown(Shutdown::Both);
                    break;
                }
                Ok(n) => {
                    bytes_transferred += target_bytes;
                    target_bytes = 0;
                    //logging::event_log(
                    //    format!(
                    //        "{} bytes writte to {}",
                    //        n,
                    //        s_stream.local_addr().unwrap().ip()
                    //    )
                    //    .as_str(),
                    //);
                }
                Err(ProxyError::IOBlocked) => {}
                Err(e) => {
                    t_stream.shutdown(Shutdown::Both);
                    break;
                }
            };
        }
    }

    bytes_transferred
}

pub fn process_connection(stream: &mut TcpStream) -> Result<()> {
    let req_type = get_req_type(stream);

    match req_type {
        Ok(ReqType::CONNECT(p)) => {
            logging::event_log(&format!("[CONNECT Request] for {}", p));

            let mut t_stream = get_target_stream(&p)?;
            t_stream.set_nodelay(true);
            let res = write_to_tcpstream(stream, HTTP_OK)?;

            logging::event_log(&format!("[Connection established]"));

            let n = tunnel_bytes(stream, &mut t_stream);
            logging::event_log(&format!("[Tunnel] {} bytes exchanged", n));
        }

        Ok(ReqType::GET(p)) => {
            logging::event_log(&format!("[GET Request] for {}", p));
        }

        Err(e) => {
            logging::event_log(&format!("[{:?}] while parsing request", e));
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_req_handler {}
