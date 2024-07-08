use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;

mod handlers;
mod models;
mod state;

use handlers::author_handlers::{add_author, get_all, get_author};
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb+srv://Rust-Server:Rust-Server@rustserver.a5c2xn3.mongodb.net/?retryWrites=true&w=majority&appName=RustServer").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let state = web::Data::new(Mutex::new(AppState { client }));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(add_author)
            .service(get_all)
            .service(get_author)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
