use sqlx::{Connection, PgPool};
use std::net::TcpListener;

use z2p::configuration::get_configuration;
use z2p::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("could not connect to postgres");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("unable to bind to port 8080");
    run(listener, connection_pool)?.await
}
