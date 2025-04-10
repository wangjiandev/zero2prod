use std::io::Error;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
   run()?.await
}

