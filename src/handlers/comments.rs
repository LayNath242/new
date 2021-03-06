use actix_web::{HttpResponse, Result, web, HttpRequest};
use crate::handlers::pg_pool_handler;
use crate::db_connection::PgPool;
use crate::models::comment::{Comment, NewComment, Commentlist};
// use crate::handlers::LoggedUser;

pub fn index(_user: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(Commentlist::list(&pg_pool)))
}

pub fn create(_user: HttpRequest, new_comment: web::Json<NewComment> ,pool: web::Data<PgPool>) ->Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    new_comment
        .create(&pg_pool)
        .map(|comment| HttpResponse::Ok().json(comment))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn show(_user: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse>{
    let pg_pool = pg_pool_handler(pool)?;
    Comment::find(&id, &pg_pool)
        .map(|comment| HttpResponse::Ok().json(comment))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })

}


pub fn destroy(_user: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Comment::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn update(_user: HttpRequest, 
            id: web::Path<i32>, 
            new_comment: web::Json<NewComment>, 
            pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Comment::update(&id, &new_comment, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}