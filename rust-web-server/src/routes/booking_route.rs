use actix_web::{post,put, delete, web::{Data, Json, Path}, get, HttpResponse};
use crate::{models::booking_models::{Booking, BookingRequest}, services::db::Database};

#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse{
    match db.create_booking(
        Booking::try_from(BookingRequest{
            owner: request.owner.clone(), 
            start_time: request.start_time.clone(), 
            duration: request.duration.clone(),
        })
        .expect("Failed to create booking")
    ).await{
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/booking")]
pub async fn get_bookings(db: Data<Database>) -> HttpResponse{
    let bookings = db.get_bookings().await.expect("Failed to get bookings");
    HttpResponse::Ok().json(bookings)
}  

#[get("/booking/{id}")]
pub async fn get_booking(db: Data<Database>, id: Path<String>) -> HttpResponse{
    let booking = db.get_booking(id.into_inner()).await.expect("Failed to get booking");
    HttpResponse::Ok().json(booking)
}   

#[delete("/booking/{id}")]
pub async fn delete_booking(db: Data<Database>, id: Path<String>) -> HttpResponse{
    let booking = db.delete_booking(id.into_inner()).await.expect("Failed to delete booking");
    HttpResponse::Ok().json(booking)
}

#[put("/booking/{id}")]
pub async fn update_booking(db: Data<Database>, id: Path<String>, request: Json<BookingRequest>) -> HttpResponse{
    let booking = db.update_booking(id.into_inner(), request.into_inner()).await.expect("Failed to update booking");
    HttpResponse::Ok().json(booking)
}

 

