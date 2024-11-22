use actix_web::{web, App, HttpServer, Responder};
mod models;
mod handlers;

async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .service(
                web::scope("/api")
                    .route("/items", web::post().to(handlers::create_item))
                    .route("/items", web::get().to(handlers::get_items))
                    .route("/items/{id}", web::get().to(handlers::get_item))
                    .route("/items/{id}", web::put().to(handlers::update_item))
                    .route("/items/{id}", web::delete().to(handlers::delete_item))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
