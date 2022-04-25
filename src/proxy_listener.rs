use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use crate::firewall::Firewall;
use crate::logging;
use crate::request_handler::process_connection;

// Req Handling error type
pub type Result<T> = std::result::Result<T, ProxyError>;

#[derive(Debug)]
pub enum ProxyError {
    IO(String),
    Parse(String),
    Other(String),
    StreamClosed,
    CannotConnectToDest,
    IOBlocked,
}

/// Open connection to the target and return the tcp stream
pub fn get_target_stream(addr: &str) -> Result<TcpStream> {
    match TcpStream::connect(addr) {
        Ok(a) => Ok(a),
        Err(_) => Err(ProxyError::CannotConnectToDest),
    }
}

// Create a simple TcpListener for given ip and port
fn get_listener(ip: &String, port: &String) -> TcpListener {
    // Will remove all debugging lines after testing
    println!("Starting proxy listener on {}:{}", ip, port);

    let listener = TcpListener::bind(format!("{}:{}", ip, port));
    let listener_handler = match listener {
        Ok(sock) => sock,
        Err(err) => {
            eprintln!("Could not open listener on {}:{}", ip, port);
            eprintln!("Encountered error {}", err);
            panic!("Could not start server!");
        }
    };

    println!("Listener started");

    listener_handler
}

fn whitelist_check(addr: &SocketAddr, fwall: &mut Firewall) -> bool {
    match fwall.in_whitelist(addr.ip().to_string().as_str()) {
        true => {
            logging::event_log(
                format!("[Connection event] Got request from {:?}", addr.ip()).as_str(),
            );
            true
        }

        false => {
            logging::event_log(
                format!("[Firewall event] {} not in whitelist!", addr.ip()).as_str(),
            );

            false
        }
    }
}

pub fn run_listener() {
    let ip = String::from("127.0.0.1");
    let port = String::from("7878");

    let listener = get_listener(&ip, &port);
    let mut firewall = Firewall::new();

    for mut stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    match process_connection(&mut stream) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Got error: {:?}", e);
                        }
                    };
                });
            }

            Err(e) => {
                logging::event_log(
                    format!("[Connection event] Could not accept connection {:?}", e).as_str(),
                );
            }
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
}
