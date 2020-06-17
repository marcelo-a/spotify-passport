// #![allow(unused_imports)]
use actix_web::{web, App, HttpServer};
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod route;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(route::login))
            .route("/render", web::get().to(route::render))
            .route("/again", web::get().to(route::again))
            .route("/run", web::get().to(route::run))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}