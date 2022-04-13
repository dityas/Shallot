mod logging;
mod proxy_listener;
mod request_handler;
mod firewall;

use tokio::net::TcpListener;
use tokio::runtime::Runtime;

fn main() {

    // Create runtime for async
    let tokio_runtime = match Runtime::new() {

        Ok(rt) => {
            println!("Spawned new tokio runtime");
            rt
        },

        Err(e) => {
            panic!("[!!!] Could not spawn tokio runtime: {}", e);
        }
    };

    // Block the runtime on the proxy listener
    tokio_runtime.block_on(async {
        match proxy_listener::run_listener().await {
            Ok(res) => {
                println!("Server shutdown gracefully");
            },

            Err(e) => {
                eprintln!("Fatal Error!");
            }
        };
    });

    println!("Terminating server!");
}

#[cfg(test)]
mod test_main_runtime {

    use tokio::runtime::Runtime;

    #[test]
    fn test_listener_init() -> Result<(), std::io::Error> {

        let _trt = Runtime::new()?;
        println!("Successfully created tokio runtime {:?}", _trt);
        Ok(())
    }

}
