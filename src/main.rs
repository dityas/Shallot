mod logging;
mod proxy_listener;

fn run_server() { 
    proxy_listener::run_listener()
}

fn main() {
    run_server();
}
