use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

#[get("/ping")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "pong"
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
          .wrap(Logger::default())
          .service(pong)
    })
      .bind(("127.0.0.1", 3000))?
      .run()
      .await
}