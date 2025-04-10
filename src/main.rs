use std::{io::Error, net::TcpListener};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let lstener = TcpListener::bind(address)?;
    run(lstener)?.await
}
