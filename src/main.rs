// src/main.rs
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Docker!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:5000");
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}