use std::net::TcpListener;

use zero2prod::{
    configuration::{self, get_configuration},
    startup::run,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    run(listener)?.await
}
