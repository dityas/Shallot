use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use std::io::Read;
use std::net::SocketAddr;

use crate::logging;
use crate::request_handler;
use crate::request_handler::HTTPReq;
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
//    // Get addr
//    let addr = stream.peer_addr().map_err(|e| ReqHandleError::IOError(e.to_string()))?;
//
//    // Read bytes into a buffer
//    let mut request: [u8 ; 4096] = [0 ; 4096];
//    let req_bytes = stream
//        .read(&mut request)
//        .map_err(|e| ReqHandleError::IOError(e.to_string()))?;
//
//    //let mut req = Request::new(&headers);
//
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


async fn process_request(mut stream: &TcpStream, addr: SocketAddr) {

    // Log connection event
    logging::event_log(
        format!("[Connection event] Got request from {:?}", addr).as_str());

    println!("Addr is {}", addr.ip().to_string());

}

pub async fn run_listener() -> Result<()>  {

    let ip = String::from("127.0.0.1");
    let port = String::from("7878");

    let listener = get_listener(&ip, &port).await;
    let mut firewall = Firewall::new();

    loop {

        let (mut stream, addr) = listener.accept().await
            .map_err(|e| ProxyError::IOError(e.to_string()))?;

        let verif = firewall_checks(&addr, &mut firewall);

        let _task = task::spawn(async move {
            process_request(&mut stream, addr).await
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
