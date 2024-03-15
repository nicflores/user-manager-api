use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

use usermsgsvc::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from a `.env` file if present
    dotenv().ok();

    // Configure logging
    if std::env::var("RUST_LOG").is_err() {
        // Default to info level if RUST_LOG isn't set
        std::env::set_var("RUST_LOG", "info");
    }
    //tracing_subscriber::fmt::init();

    // Read the server address from environment variables or default to 127.0.0.1:3000
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let addr: SocketAddr = addr.parse().expect("Unable to parse SERVER_ADDR");

    // Start the server
    println!("Starting server at {}", addr);
    run(addr).await
}
