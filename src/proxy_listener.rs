use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use crate::firewall::Firewall;
use crate::logging;
use crate::logging::Event;
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
    logging::event_log(
        Event::ProxyServer,
        &format!("Starting proxy listener on {}:{}", ip, port),
    );

    let listener = TcpListener::bind(format!("{}:{}", ip, port));
    let listener_handler = match listener {
        Ok(sock) => sock,
        Err(err) => {
            logging::event_log(Event::ProxyServer, &format!("Encountered error {}", err));
            panic!("Could not start server!");
        }
    };

    logging::event_log(Event::ProxyServer, "Listener started");

    listener_handler
}

fn whitelist_check(addr: &SocketAddr, fwall: &mut Firewall) -> bool {
    fwall.in_whitelist(addr.ip().to_string().as_str())
}

pub fn run_listener() {
    let ip = String::from("127.0.0.1");
    let port = String::from("7878");

    let listener = get_listener(&ip, &port);
    let mut firewall = Firewall::new();

    for mut stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let ip = stream.peer_addr().unwrap();

                if !whitelist_check(&ip, &mut firewall) {
                    logging::event_log(Event::WhiteListDeny, &format!("{} not in whitelist", ip));
                } else {
                    thread::spawn(move || {
                        match process_connection(&mut stream) {
                            Ok(_) => {}
                            Err(e) => {
                                logging::event_log(
                                    Event::Connection,
                                    &format!("Got error: {:?}", e),
                                );
                            }
                        };
                    });
                }
            }

            Err(e) => {
                logging::event_log(
                    Event::Connection,
                    &format!("Could not accept connection {:?}", e),
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
