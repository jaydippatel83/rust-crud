mod models;
mod services;
mod routes;

use actix_web::{HttpServer, App, Responder, HttpResponse, get, web::Data    };
use routes::{
    owner_route::{create_owner},
    dog_route::{create_dog},
    booking_route::{create_booking, get_bookings}
};
use services::db::Database; 


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Jaydip!")
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    let database = Database::init().await;
    let db_data = Data::new(database);

   HttpServer::new(move || {
    App::new()
    .app_data(db_data.clone())
    .service(hello)
    .service(create_owner)
    .service(create_dog)
    .service(create_booking)
    .service(get_bookings)  
   })
   .bind("localhost:8080")?
   .run()
   .await
}
 