use actix_web::web;
use actix_web::{App, HttpServer, dev::Server};
use sqlx::PgPool;
use std::{io::Error, net::TcpListener};

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(subscribe)
            .service(health_check)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
