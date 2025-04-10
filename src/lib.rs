use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::io::Error;

pub fn run() -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8080")?
        .run();
    Ok(server)
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
