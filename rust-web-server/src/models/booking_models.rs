use std::{convert::TryFrom, time::SystemTime};

use super::{dog_models::Dog, owner_model::Owner};
use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking { 
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration: i32, 
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBooking {
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration: i32, 
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration: i32,  
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(request: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_datetime : SystemTime = chrono::DateTime::parse_from_rfc3339(&request.start_time)
        .map_err(|e| format!("Failed to parse start time: {}", e))?
        .with_timezone(&Utc)
        .into(); 

        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&request.owner).expect("Failed to parse owner"),
            start_time: DateTime::from(chrono_datetime), 
            duration: request.duration,
            cancelled: false,
        })
    }
}