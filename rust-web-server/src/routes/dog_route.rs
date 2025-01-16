use actix_web::{post, web::{Data, Json}, get, HttpResponse, Responder};
use crate::{models::dog_models::{Dog, DogRequest}, services::db::Database};

#[post("/dog")]
pub async fn create_dog(db: Data<Database>, request: Json<DogRequest>) -> HttpResponse{
    match db.create_dog(
        Dog::try_from(DogRequest{
            name: request.name.clone(),
            breed: request.breed.clone(),
            age: request.age.clone(),
            owner: request.owner.clone(),
        })
        .expect("Failed to create dog")
    ).await{
        Ok(dog) => HttpResponse::Ok().json(dog),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}