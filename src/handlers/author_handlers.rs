use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Mutex;

use crate::models::author::{ApiResponse, Author, AuthorDb, UpdateAuthor};
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
pub async fn get_author(
    data: web::Data<Mutex<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
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
        Err(error) => {
            HttpResponse::InternalServerError().body(format!("Failed to get author: {}", error))
        }
    }
}

#[get("/authors")]
pub async fn get_all(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<AuthorDb>("authors");

    let authors = match collection.find(doc! {}).await {
        Ok(cursor) => cursor
            .try_collect::<Vec<AuthorDb>>()
            .await
            .unwrap_or_else(|e| {
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

#[delete("/author/{id}")]
pub async fn delete_author(
    data: web::Data<Mutex<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<AuthorDb>("authors");

    let object_id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Format"),
    };

    let author_deleted = match collection.delete_one(doc! { "_id": object_id }).await {
        Ok(result) => result,
        Err(e) => {
            println!("failed to delete author: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    if author_deleted.deleted_count == 1 {
        HttpResponse::Ok().body("Author deleted successfully")
    } else {
        HttpResponse::NotFound().body("Author not found")
    }
}

#[put("/author/{id}")]
pub async fn update_author(
    data: web::Data<Mutex<AppState>>,
    path: web::Path<String>,
    updated_author: web::Json<UpdateAuthor>,
) -> impl Responder {
    let client = &data.lock().unwrap().client;
    let db = client.database("rustserver");
    let collection = db.collection::<AuthorDb>("authors");

    let object_id = match ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Format"),
    };

    let mut update_doc = doc! {};

    if let Some(name) = &updated_author.name {
        update_doc.insert("name", name);
    }

    if let Some(last_name) = &updated_author.last_name {
        update_doc.insert("last_name", last_name);
    }
    if let Some(birth_date) = &updated_author.birth_date {
        update_doc.insert("birth_date", birth_date);
    }

    if let Some(books) = &updated_author.books {
        let books_bson = match mongodb::bson::to_bson(&books) {
            Ok(bson) => bson,
            Err(_) => return HttpResponse::BadRequest().body("Failed t format Format"),
        };
        update_doc.insert("books", books_bson);
    }

    if update_doc.is_empty() {
        return HttpResponse::BadRequest().body("No fields to update");
    }

    let update_result = collection
        .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc })
        .await;

    // Handle the result of the update
    match update_result {
        Ok(result) => {
            if result.matched_count == 1 {
                HttpResponse::Ok().body("Author updated successfully")
            } else {
                HttpResponse::NotFound().body("Author not found")
            }
        }
        Err(e) => {
            println!("Failed to update author: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
