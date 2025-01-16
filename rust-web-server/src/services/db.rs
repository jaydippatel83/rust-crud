use std::{env, str::FromStr, time::SystemTime};

use chrono::Utc;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_document, oid::ObjectId, DateTime},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::booking_models::{Booking, FullBooking};
use crate::models::dog_models::Dog;
use crate::models::owner_model::Owner;
pub struct Database{
    booking: Collection<Booking>,
    dog: Collection<Dog>,
    owner: Collection<Owner>,
}
#[allow(unused_imports)]
impl Database {
    pub async fn init()-> Self{
        let uri = match std::env::var("MONGODB_URI"){
            Ok(v) => v.to_string(),
            Err(_) => panic!("MONGODB_URI not found in environment variables"),
        };

        let client = Client::with_uri_str(&uri).await.unwrap();
        let db = client.database("dog_park");
        let booking: Collection<Booking> = db.collection("booking");
        let dog: Collection<Dog> = db.collection("dog");
        let owner: Collection<Owner> = db.collection("owner");
        Database { booking, dog, owner }
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<(InsertOneResult), Error>{
        let result  = self.owner.insert_one(owner).await.ok().expect("Failed to create owner");
        Ok(result)
    }
    pub async fn create_dog(&self, dog: Dog) -> Result<(InsertOneResult), Error>{
        let result  = self.dog.insert_one(dog).await.ok().expect("Failed to create dog");
        Ok(result)
    }
    pub async fn create_booking(&self, booking: Booking) -> Result<(InsertOneResult), Error>{
        let result  = self.booking.insert_one(booking).await.ok().expect("Failed to create booking");
        Ok(result)
    }     

    pub async fn can_book(&self, booking_id: &str) -> Result<(UpdateResult), Error>{
        let result  = self.booking.update_one(
            doc!{"_id": ObjectId::from_str(booking_id).expect("Failed to convert booking_id to ObjectId")},
            doc!{"$set": doc!{"cancelled": true}} 
        ).await.ok().expect("Failed to create booking");
        Ok(result)
    }

    pub async fn get_booking(&self, booking_id: &str) -> Result<Booking, Error>{
        let result = self.booking.find_one(doc!{"_id": ObjectId::from_str(booking_id).expect("Failed to convert booking_id to ObjectId")}, None).await.ok().expect("Failed to get booking");
        Ok(result)
    }

    pub async fn delete_booking(&self, booking_id: &str) -> Result<Booking, Error>{
        let result = self.booking.delete_one(doc!{"_id": ObjectId::from_str(booking_id).expect("Failed to convert booking_id to ObjectId")}, None).await.ok().expect("Failed to delete booking");
        Ok(result)
    }

    pub async fn update_booking(&self, booking_id: &str, booking: Booking) -> Result<Booking, Error>{
        let result = self.booking.update_one(doc!{"_id": ObjectId::from_str(booking_id).expect("Failed to convert booking_id to ObjectId")}, doc!{"$set": booking}, None).await.ok().expect("Failed to update booking");
        Ok(result)
    }



    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error>{
        let now: SystemTime = Utc::now().into();
        let mut results  = self.booking.aggregate(vec![
            doc!{
                "$match": {
                    "cancelled": false,
                    "start_time": {
                        "$gte": DateTime::from_system_time(now)
                    }
                },
            },
            doc!{
                "$lookup": doc!{
                    "from": "owner",
                    "localField": "owner",
                    "foreignField": "_id",
                    "as": "owner"
                }   
            },
            doc!{
                "$unwind": doc!{"path": "$owner"}
            },
            doc!{
                "$lookup": doc!{
                    "from": "dog",
                    "localField": "owner._id",
                    "foreignField": "owner",
                    "as": "dogs"
                }
            }, 
        ]).await.ok().expect("Failed to get bookings"); 

        let mut bookings: Vec<FullBooking> = Vec::new();
         while let Some(result) = results.next().await{
           match result{
            Ok(doc) => {
                let booking: FullBooking = from_document(doc).expect("Failed to convert document to FullBooking");
                bookings.push(booking);
            }
            Err(e) => panic!("Failed to get bookings: {}", e),
           }
         }
        Ok(bookings)
    } 
 
}
