use actix_web::{post, web::{Data, Json}, get, HttpResponse, Responder};
use crate::{models::owner_model::{Owner, OwnerRequest}, services::db::Database};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse{
    match db.create_owner(
        Owner::try_from(OwnerRequest{
            name: request.name.clone(),
            email: request.email.clone(),
            phone: request.phone.clone(),
            address: request.address.clone(),
        })
        .expect("Failed to create owner")
    ).await{
        Ok(owner) => HttpResponse::Ok().json(owner),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}