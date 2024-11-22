use actix_web::{web, App, HttpServer, Responder};
mod fibonacci;
mod models;

async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(hello)).service(
            web::scope("/api").route("/fibonacci/{n}", web::get().to(fibonacci::fibonacci)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
