use std::fmt::Debug;
use std::io::ErrorKind::WouldBlock;
use std::io::Read;
use std::io::Write;

use std::sync::Arc;
use std::sync::Mutex;

use std::net::Shutdown;
use std::net::TcpStream;

use httparse::{Request, EMPTY_HEADER};

use std::str;

use crate::firewall::Firewall;
use crate::logging;
use crate::logging::Event;
use crate::proxy_listener::get_target_stream;
use crate::proxy_listener::ProxyError;
use crate::proxy_listener::Result;

/// HTTP responses from the proxy server
/// HTTP response for 200 OK
const HTTP_OK: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
const HTTP_NOT_AUTH: &[u8] = "HTTP/1.1 403 Forbidden\r\n\r\n".as_bytes();

/// Parse request into a well defined request type
/// For now, the proxy only supports GET and CONNECT requests
#[derive(Debug)]
pub enum ReqType {
    CONNECT(String),
    GET(String),
}

/// Determine request type. Get the raw request and parse it into
/// a ReqType.
fn determine_request(buf: &[u8]) -> Result<ReqType> {
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

/// Forward data back and forth between source and target using the TunnelBuffer struct
struct TunnelBuffer(usize, [u8; 10240]);

fn convert_tunnel_buffer(buf: TunnelBuffer) -> Result<String> {
    match str::from_utf8(&buf.1[0..buf.0]) {
        Ok(res) => Ok(res.to_owned()),
        _ => Err(ProxyError::Parse(
            "Could not parse UTF-8 string.".to_owned(),
        )),
    }
}

fn tunnel_through(
    tunnel_buf: &mut TunnelBuffer,
    src: &mut TcpStream,
    dst: &mut TcpStream,
) -> Result<usize> {
    let mut result = Ok(0usize);

    // if tunnel buffer is empty, read from src
    if tunnel_buf.0 == 0 {
        match read_from_tcpstream(src, &mut tunnel_buf.1) {
            Ok(0) => {
                result = Err(ProxyError::StreamClosed);
            }
            Ok(n) => {
                tunnel_buf.0 = n;
            }
            Err(ProxyError::IOBlocked) => {}
            Err(_) => {
                result = Err(ProxyError::StreamClosed);
            }
        };
    }

    // if tunnel buffer was modified, write it to dst
    if tunnel_buf.0 > 0 {
        match write_to_tcpstream(dst, &mut tunnel_buf.1[0..tunnel_buf.0]) {
            Ok(0) => {
                result = Err(ProxyError::StreamClosed);
            }
            Ok(n) => {
                logging::event_log(
                    Event::DataTransfer,
                    &format!(
                        "{} of {} bytes sent from {} to {}",
                        n,
                        tunnel_buf.0,
                        src.peer_addr()
                            .map_err(|_| ProxyError::Other("".to_owned()))?
                            .ip(),
                        dst.peer_addr()
                            .map_err(|_| ProxyError::Other("".to_owned()))?
                            .ip()
                    ),
                );

                result = Ok(n);
                tunnel_buf.0 = 0;
            }
            Err(ProxyError::IOBlocked) => {}
            Err(_) => {
                result = Err(ProxyError::StreamClosed);
            }
        };
    }

    result
}

fn tunnel(s_stream: &mut TcpStream, t_stream: &mut TcpStream) -> usize {
    let mut total_bytes = 0usize;

    // Init buffers for tunneling
    let mut source_buf = TunnelBuffer(0usize, [0; 10240]);
    let mut target_buf = TunnelBuffer(0usize, [0; 10240]);

    // Set both streams to non blocking
    s_stream.set_nonblocking(true);
    t_stream.set_nonblocking(true);

    loop {
        match tunnel_through(&mut source_buf, s_stream, t_stream) {
            Ok(n) => {
                total_bytes += n;
            }
            Err(_) => {
                s_stream.shutdown(Shutdown::Both);
                t_stream.shutdown(Shutdown::Both);
                break;
            }
        };

        match tunnel_through(&mut target_buf, t_stream, s_stream) {
            Ok(n) => {
                total_bytes += n;
            }
            Err(_) => {
                s_stream.shutdown(Shutdown::Both);
                t_stream.shutdown(Shutdown::Both);
                break;
            }
        };
    }

    total_bytes
}

pub fn process_connection(stream: &mut TcpStream, fwall: Arc<Mutex<Firewall>>) -> Result<()> {
    let mut _fwall = fwall.lock().unwrap();

    let src_addr = stream
        .peer_addr()
        .map_err(|e| ProxyError::Other(format!("{:?}", e)))?
        .ip();

    let req_type = get_req_type(stream);

    match req_type {
        Ok(ReqType::CONNECT(p)) => {
            logging::event_log(
                Event::Connection,
                &format!("CONNECT request for {} from {}", p, src_addr),
            );

            let mut t_stream = get_target_stream(&p)?;
            let dst_addr = t_stream
                .peer_addr()
                .map_err(|e| ProxyError::Other(format!("{:?}", e)))?
                .ip();

            match _fwall.in_whitelist(&src_addr.to_string()) {
                true => match _fwall.in_blacklist(&dst_addr.to_string()) {
                    false => {
                        logging::event_log(
                            Event::ProxyServer,
                            &format!("{} and {} verified", src_addr, dst_addr),
                        );
                    }

                    true => {
                        logging::event_log(
                            Event::BlackListDeny,
                            &format!("{} in blacklist", dst_addr),
                        );
                        let _res = write_to_tcpstream(stream, HTTP_NOT_AUTH)?;
                        return Err(ProxyError::BlackListDeny);
                    }
                },

                false => {
                    logging::event_log(
                        Event::WhiteListDeny,
                        &format!("{} not in whitelist", src_addr),
                    );
                    let _res = write_to_tcpstream(stream, HTTP_NOT_AUTH)?;
                    return Err(ProxyError::WhiteListDeny);
                }
            };

            std::mem::drop(_fwall);

            // Respond with 200 OK
            let _res = write_to_tcpstream(stream, HTTP_OK)?;

            logging::event_log(
                Event::Connection,
                &format!(
                    "CONNECT tunnel established between {} and {}",
                    src_addr, dst_addr
                ),
            );

            let n = tunnel(stream, &mut t_stream);
            logging::event_log(
                Event::DataTransfer,
                &format!(
                    "Total {} bytes exchanged between {} and {}",
                    n, src_addr, dst_addr
                ),
            );

            Ok(())
        }

        Ok(ReqType::GET(p)) => {
            logging::event_log(
                Event::Connection,
                &format!("GET for {} from {}", p, src_addr),
            );
            Ok(())
        }

        Err(e) => {
            logging::event_log(
                Event::Connection,
                &format!("[{:?}] while parsing request", e),
            );
            Err(e)
        }
    }
}

#[cfg(test)]
mod test_req_handler {}
