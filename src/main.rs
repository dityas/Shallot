mod firewall;
mod logging;
mod proxy_listener;
mod request_handler;

fn main() {
    // Block the runtime on the proxy listener
    proxy_listener::run_listener();
    println!("Terminating server!");
}

#[cfg(test)]
mod test_main_runtime {}
