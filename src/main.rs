use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;

mod handlers;
mod models;
mod state;

use handlers::author_handlers::{add_author, get_all};
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let state = web::Data::new(Mutex::new(AppState { client }));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(add_author)
            .service(get_all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
