use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Docker!")
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();
    println!("Connected to database!");

    println!("Server running at http://localhost:5000");
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
