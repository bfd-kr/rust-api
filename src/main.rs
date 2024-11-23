use actix_web::{web, App, HttpServer};
mod fibonacci;
mod fibonacci_kbytes;
mod models;
mod random_kbytes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/fibonacci/{n}", web::get().to(fibonacci::fibonacci))
                .route(
                    "/random_kbytes/{n}",
                    web::get().to(random_kbytes::random_kbytes),
                )
                .route(
                    "/fibonacci_kbytes/{f}/{k}",
                    web::get().to(fibonacci_kbytes::fibonacci_kbytes),
                ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
