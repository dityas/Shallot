use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use hyper::{server::conn::Http, service::service_fn};

use std::io::Read;
use std::net::SocketAddr;
use std::rc::Rc;

use crate::logging;
use crate::request_handler::handle_request;
use crate::firewall::Firewall;


// Req Handling error type
pub type Result<T> = std::result::Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {

    IOError(String),
    ParseError(String),
    OtherError(String),
}

// Create a simple TcpListener for given ip and port
async fn get_listener(ip: &String, port: &String) -> TcpListener {

    // Will remove all debugging lines after testing
    println!("Starting proxy listener on {}:{}", ip, port);

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await;
    let listener_handler = match listener {

        Ok(sock) => sock,
        Err(err) => {
            eprintln!("Could not open listener on {}:{}", ip, port);
            eprintln!("Encountered error {}", err);
            panic!("Could not start server!");
        },
    };

    println!("Listener started");

    listener_handler
}


//fn parse_request(mut stream: &TcpStream) -> Result<()> {
//
//    let mut _stream = stream.into_std()
//        .map_err(|e| ProxyError::OtherError("Could not convert to std stream"
//                .to_string()))?;
//
//    _stream.set_nonblocking(false)
//        .map_err(|e| ProxyError::OtherError("Could not set NON_BLOCKING"
//                .to_string()))?;
//
//    // Read bytes into a buffer
//    let mut request = Vec::new();
//    _stream
//        .read_to_end(&mut request)
//        .map_err(|e| ProxyError::IOError(e.to_string()))?;
//
//    //let mut req = Request::new(&headers);
//
//    println!("Got {:?}", request);
//
//    // Directly parse the request
//    //let req_string = String::from_utf8(&request[0..req_bytes])
//    //    .map_err(|e| ReqHandleError::ParseError(e.to_string()))?;
//    //let req_struct = request_handler::get_req_struct(&req_string)
//    //    .ok_or(ReqHandleError::ParseError("While making req_struct".to_string()))?;
//
//    // Log successful connections
//    //println!("Got request {:?} from {:?}", req_struct, addr);
//    //logging::event_log(format!("[+] Verified Request of type {:?} from {:?}", 
//    //        req_struct.req_type, addr).as_str());
//
//    // return parsed struct
//    Ok(())
//}


//fn forward_request(req: &HTTPReq) -> Result<()> {
//
//
//}

fn firewall_checks(addr: &SocketAddr, fwall: &mut Firewall) -> bool {

    match fwall.in_whitelist(addr.ip().to_string().as_str()) {

        true => {
            logging::event_log(
                format!("[Connection event] Got request from {:?}", addr).as_str());
            true
        },

        false => {
            logging::event_log(
                format!("[Firewall event] {:?} not in whitelist!", addr).as_str());

            false
        },
    }
}


async fn process_request(stream: TcpStream, addr: SocketAddr) {
   
    let b = hyper::body::Body::from(stream);
    // println!("{:?}", b);
    Http::new().serve_connection(stream, service_fn(handle_request)).await;
}

pub async fn run_listener() -> Result<()>  {

    let ip = String::from("127.0.0.1");
    let port = String::from("7878");

    let listener = get_listener(&ip, &port).await;
    let mut firewall = Firewall::new();

    loop {

        let (stream, addr) = listener.accept().await
            .map_err(|e| ProxyError::IOError(e.to_string()))?;

        let verif = firewall_checks(&addr, &mut firewall);

        let _task = task::spawn(async move {
            process_request(stream, addr).await
        });

        let outcome = _task.await.map_err(|e| ProxyError::OtherError(e.to_string()))?;
    }
}

#[cfg(test)]
mod test_proxy_listener {

    use super::get_listener;
    use super::run_listener;

    #[test]
    fn test_listener_init() {

        let ip: String = String::from("127.0.0.1");
        let port: String = String::from("8080");

        let result = get_listener(&ip, &port);
    }

    #[test]
    fn test_hyper_server() {


    }
}
