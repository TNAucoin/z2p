use std::net::TcpListener;

use z2p::startup::run;
use z2p::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind(
        format!("127.0.0.1:{}",configuration.application_port)
    ).expect("unable to bind to port 8080");
    run(listener)?.await
}
