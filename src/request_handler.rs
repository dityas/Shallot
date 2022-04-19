
use std::future::Future;
use std::convert::Infallible;
use std::fmt::Display;

use tokio::net::TcpStream;
use httparse::{EMPTY_HEADER, Request};

use crate::logging;
use crate::proxy_listener::Result;
use crate::proxy_listener::ProxyError;


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
async fn determine_request(buf: &[u8]) -> Result<ReqType> {

    let len = buf.len();
    let mut headers = [EMPTY_HEADER; 4096];
    let mut req = Request::new(&mut headers);
    let res = req.parse(&buf);

    match req.method {
        Some("CONNECT") => match req.path {
            Some(a) => Ok(ReqType::CONNECT(a.to_owned())),
            None => Err(ProxyError::ParseError("Could not parse path".to_owned()))
        },

        Some("GET") => match req.path {
            Some(p) => Ok(ReqType::GET(p.to_owned())),
            None => Err(ProxyError::ParseError("Could not parse path".to_owned())),
        },

        _ => Err(ProxyError::ParseError("Could not parse request".to_owned())),
    }
}

async fn process_tcpstream_with<'a, T: Display, F, Fut>(
    stream: &TcpStream,
    buf: &'a mut [u8],
    f: F) -> Result<T> 

where
    F: FnOnce(&'a [u8]) -> Fut,
    Fut: Future<Output = Result<T>> + 'a {

        stream.readable().await;
        match stream.try_read(buf) {

            Ok(0) => Err(ProxyError::IOError("0 bytes read from stream".to_owned())),
            Ok(n) => f(buf).await,
            Err(e) => Err(ProxyError::IOError(format!("Error: {:?}", e))),
        }
}

pub async fn process_request(stream: &TcpStream) {

    let req_type = process_tcpstream_with(
        stream, &mut [0u8; 4096], determine_request).await;

    match req_type {
        Ok(ReqType::CONNECT(p)) => {
            logging::event_log(&format!("[CONNECT Request] for {}", p));
        },
        
        Ok(ReqType::GET(p)) => {
            logging::event_log(&format!("[GET Request] for {}", p));
        },
        
        Err(e) => {
            logging::event_log(&format!("[{:?}] while parsing request", e));
        },
    }
}

#[cfg(test)]
mod test_req_handler {


}
