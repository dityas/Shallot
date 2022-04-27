mod firewall;
mod logging;
mod proxy_listener;
mod request_handler;
mod statistics;

use std::thread;
use statistics::generate_statistics;
fn main() {
    // Run generate_statistics in a background thread which generates the stats from event_log.txt
    // every 5s seconds into statistics.txt file.
    thread::spawn(|| {
        generate_statistics();
    });
    // Block the runtime on the proxy listener
    proxy_listener::run_listener();
    println!("Terminating server!");
}

#[cfg(test)]
mod test_main_runtime {}
