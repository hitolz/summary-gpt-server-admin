use actix_web::{
    get,
    web::{self},
    HttpResponse, Scope,
};

use crate::api::success;

///请求路由
pub fn routes() -> Scope {
    web::scope("/admin").service(index)
}

#[get("")]
pub async fn index() -> HttpResponse {
    success(Some(1))
}
