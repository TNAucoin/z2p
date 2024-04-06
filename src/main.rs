use std::net::TcpListener;

use z2p::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("unable to bind to port 8080");
    run(listener)?.await
}
