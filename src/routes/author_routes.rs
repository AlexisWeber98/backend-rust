use actix_web::{get, post, web, HttpResponse, Responder};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct AppState {
    pub client: Client,
}

#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    last_name: String,
    birth_date: String,
    books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorDb {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub last_name: String,
    pub birth_date: String,
    pub books: Vec<Book>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Book {
    title: String,
    description: String,
    pages: u8,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    result: String,
    data: Author,
}

#[post("/author")]
pub async fn add_author(
    data: web::Data<Mutex<AppState>>,
    new_author: web::Json<Author>,
) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<Author>("authors");

    let new_author_inner = new_author.into_inner();

    match collection.insert_one(&new_author_inner, None).await {
        Ok(_) => {
            let response = ApiResponse {
                result: "success".to_string(),
                data: new_author_inner,
            };
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            HttpResponse::InternalServerError().body(format!("Failed to add author: {}", error))
        }
    }
}

#[get("/authors")]
pub async fn get_all(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<Author>("authors");

    match collection.find(None, None).await {
        Ok(cursor) => {
            let authors: Vec<Author> = cursor.try_collect().await.unwrap_or_else(|e| {
                println!("Failed to get authors: {}", e);
                Vec::new()
            });

            HttpResponse::Ok().json(authors)
        }
        Err(e) => {
            println!("Failed to get authors: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
