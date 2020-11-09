use crate::{
    documents::{Post, PostUpsert},
    services::{DocumentService, Pagination, Query},
    AppState,
};

use super::super::Result;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_one(id: web::Path<String>, app: web::Data<AppState>) -> Result<impl Responder> {
    let result = app
        .post_service
        .get_one(Post::from_string_id(id.into_inner()))
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_many(
    query: web::Query<Post>,
    pagination: web::Query<Pagination>,
    app: web::Data<AppState>,
) -> Result<impl Responder> {
    let query = query.into_inner();
    let pagination = pagination.into_inner();
    let result = app.post_service.get_many(query, pagination).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn put(
    id: web::Path<String>,
    data: web::Json<PostUpsert>,
    app: web::Data<AppState>,
) -> Result<impl Responder> {
    let result = app
        .post_service
        .put(
            Post::from_string_id(id.into_inner()),
            data.into_inner().into(),
        )
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn post(data: web::Json<PostUpsert>, app: web::Data<AppState>) -> Result<impl Responder> {
    // TODO: Validate DTO
    let result = app.post_service.post(data.into_inner().into()).await?;
    Ok(HttpResponse::Created().json(result))
}

pub async fn delete(id: web::Path<String>, app: web::Data<AppState>) -> Result<impl Responder> {
    match app
        .post_service
        .delete(Post::from_string_id(id.into_inner()))
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(e) => Err(e),
    }
}
