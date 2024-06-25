use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub last_name: String,
    pub birth_date: String,
    pub books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorDb {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub last_name: String,
    pub birth_date: String,
    pub books: Vec<Book>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub description: String,
    pub pages: u8,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub result: String,
    pub data: Author,
}
