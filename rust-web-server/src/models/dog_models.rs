use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dog { 
    pub _id: ObjectId,
    pub name: String,
    pub breed: String,
    pub age: i32,
    pub owner: ObjectId,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DogRequest {
    pub name: String,
    pub breed: String,
    pub age: i32,
    pub owner: String,
}

impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;

    fn try_from(request: DogRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: request.name,
            breed: request.breed,
            age: request.age,
            owner: ObjectId::parse_str(&request.owner).expect("Failed to parse owner"),
        })
    }
}