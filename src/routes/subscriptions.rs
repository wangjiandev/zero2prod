use actix_web::{HttpResponse, Responder, post, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[post("/subscribe")]
pub async fn subscribe(_from: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
