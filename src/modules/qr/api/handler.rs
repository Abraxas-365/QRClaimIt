use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{error::AppError, modules::qr::Service};

use super::dto::QrDto;

pub async fn create_qr(service: web::Data<Arc<Service>>) -> Result<HttpResponse, AppError> {
    let qr = service.create_qr().await?;
    Ok(HttpResponse::Created().json(QrDto::from(qr)))
}

#[derive(Deserialize)]
pub struct OwnerInfo {
    id: String,
    email: String,
}
pub async fn set_owner(
    service: web::Data<Arc<Service>>,
    owner_info: web::Json<OwnerInfo>,
) -> Result<HttpResponse, AppError> {
    service
        .new_holder(&owner_info.id, &owner_info.email)
        .await?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct QrInfo {
    id: String,
    email: String,
    password: String,
}
pub async fn delete_owner(
    service: web::Data<Arc<Service>>,
    owner_info: web::Json<QrInfo>,
) -> Result<HttpResponse, AppError> {
    service
        .remove_holder(&owner_info.id, &owner_info.email, &owner_info.password)
        .await?;
    Ok(HttpResponse::Ok().finish())
}
