use actix_web::{App, HttpServer, dev::Server};
use std::{io::Error, net::TcpListener};

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(lstener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().service(subscribe).service(health_check))
        .listen(lstener)?
        .run();
    Ok(server)
}
