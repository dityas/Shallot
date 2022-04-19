use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use std::io::Read;
use std::net::SocketAddr;
use std::rc::Rc;

use httparse::{EMPTY_HEADER, Request};

use crate::logging;
use crate::firewall::Firewall;
use crate::request_handler::process_request;


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

fn whitelist_check(addr: &SocketAddr, fwall: &mut Firewall) -> bool {

    match fwall.in_whitelist(addr.ip().to_string().as_str()) {

        true => {
            logging::event_log(
                format!("[Connection event] Got request from {:?}", addr.ip()).as_str());
            true
        },

        false => {
            logging::event_log(
                format!("[Firewall event] {} not in whitelist!", addr.ip()).as_str());

            false
        },
    }
}

pub async fn run_listener() -> Result<()>  {

    let ip = String::from("127.0.0.1");
    let port = String::from("7878");

    let listener = get_listener(&ip, &port).await;
    let mut firewall = Firewall::new();

    loop {

        let (stream, addr) = listener.accept().await
            .map_err(|e| ProxyError::IOError(format!("Error: {:?}", e)))?;

        //match whitelist_check(&addr, &mut firewall) {
        match true {

            true => {
                process_request(&stream).await;
            },

            false => {
                logging::event_log(
                    format!("[Request denied] Sending reset to {}", addr.ip()).as_str());
            },
        }
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
