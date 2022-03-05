use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};

use crate::logging;
use crate::request_handler;

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
        },
    };

    println!("Listener started");

    listener_handler
}


fn handle_request(mut stream: TcpStream) -> Result<(), Error> {

    match stream.peer_addr() {

        Ok(addr) => {
            println!("Got connection from {}", addr);
            logging::log(addr);
        },

        Err(e) => {
            eprintln!("Could not get remote address, {}", e);
        },
    };

    let addr = stream.peer_addr()?;

    let mut request: [u8 ; 4096] = [0 ; 4096];
    let req_bytes = stream.read(&mut request)?;
    
    let req_string = request_handler::convert_to_string(&request[0..req_bytes]).unwrap();
    let req_struct = request_handler::get_req_struct(&req_string);
    
    println!("Got request {:?} from {:?}", req_struct, addr);


    Ok(())
}


pub fn run_listener() {

    let ip = String::from("127.0.0.1");
    let port = String::from("7878");

    let listener = get_listener(&ip, &port);

    // Wait for connections and handle them

    for stream in listener.incoming() {
       
        match stream {

            Ok(stream) => { 
                handle_request(stream);
            },

            Err(e) => {
                eprintln!("Error while processing connection, {}", e);
            },
        };
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

