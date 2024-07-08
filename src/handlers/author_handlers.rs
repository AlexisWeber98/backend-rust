use mongodb::bson::doc;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use std::sync::Mutex;

use crate::models::author::{Author, AuthorDb, ApiResponse};
use crate::state::AppState;

#[post("/author")]
pub async fn add_author(
    data: web::Data<Mutex<AppState>>,
    new_author: web::Json<Author>,
) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<Author>("authors");

    let new_author_inner = new_author.into_inner();

    match collection.insert_one(&new_author_inner).await {
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

#[get("/author/{id}")]
pub async fn get_author(data: web::Data<Mutex<AppState>>, path: web::Path<String>) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<Author>("authors");


    let object_id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    match collection.find_one(doc! { "_id": object_id }).await {
        Ok(Some(author)) => {
            let response = ApiResponse {
                result: "success".to_string(),
                data: author,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => HttpResponse::NotFound().body("Author not found"),
        Err(error) => HttpResponse::InternalServerError().body(format!("Failed to get author: {}", error)),
    }
}




#[get("/authors")]
pub async fn get_all(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<AuthorDb>("authors");

    let authors = match collection.find(doc!{}).await {
        Ok(cursor) => cursor.try_collect::<Vec<AuthorDb>>().await.unwrap_or_else(|e| {
                println!("Failed to get authors: {}", e);
                Vec::new()
            }),
            Err(e) => {
                println!("failed to get authors: {}", e);
                return HttpResponse::InternalServerError().finish();
            }

    };
    HttpResponse::Ok().json(authors)
}


