use actix_web::{HttpResponse, Scope};
use serde::ser;
use serde_derive::Serialize;


pub mod test_api;
pub mod admin;
pub mod client;

pub fn routes() -> Vec<Scope> {
   let mut scopes = vec![];
   scopes.push(test_api::routes());
   scopes.push(admin::routes());
   scopes.push(client::routes());
   scopes
}


#[derive(Serialize)]
pub struct JsonSuccess<T: ser::Serialize> {
    pub code: u32,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct JsonError {
    pub code: u32,
    pub data: Option<String>,
    pub error: Option<String>,
}

pub fn success<T: ser::Serialize>(r: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(JsonSuccess {
        code: 0,
        data: r,
        error: None,
    })
}

pub fn error(err: Option<String>) -> HttpResponse {
    HttpResponse::Ok().json(JsonError {
        code: 1,
        data: None,
        error: err,
    })
}