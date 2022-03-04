use std::net::TcpListener;


fn run_listener(ip: &String, port: &String) -> Option<TcpListener> {
    
    // Will remove all debugging lines after testing
    println!("Starting proxy listener on {}:{}", ip, port);

    let listener = TcpListener::bind(format!("{}:{}", ip, port));
    let listener_handler = match listener {

        Ok(sock) => Some(sock),
        Err(err) => {
            eprintln!("Could not open listener on {}:{}", ip, port);
            eprintln!("Encountered error {}", err);
            None
        },
    };

    // Return a Maybe TcpListener
    listener_handler
}

#[cfg(test)]
mod test_proxy_listener {
    
    use super::run_listener;

    #[test]
    fn test_listener_init() {

        let ip: String = String::from("127.0.0.1");
        let port: String = String::from("8080");

        let result = run_listener(&ip, &port);
    }
}

