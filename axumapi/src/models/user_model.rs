use chrono::NaiveDateTime;
use derive_more::From;
use entity::user;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize, Clone , From )] // for viewing data ... allows cloning
pub struct UserModel{
pub name: String,
pub email: String,
pub password: String,
pub uuid: Uuid,
pub created_at: NaiveDateTime
}

#[derive(Serialize,Deserialize )] // for viewing data ... allows cloning
pub struct UserMicroModel{
pub name: String,
pub uuid: Uuid,
}



impl UserModel {
   pub fn new(model: user::Model) -> Self {
        Self {
            name: model.name,
            email: model.email,
            password: model.password,
            uuid: model.uuid,
            created_at: model.created_at,
        }
    }
}


#[derive(Serialize,Deserialize)]
pub struct CreateUser{
pub name: String,
pub email: String,
pub password: String,

}


#[derive(Serialize,Deserialize)]
pub struct LoginUser{
pub email: String,
pub password: String,

}



#[derive(Serialize,Deserialize)]
pub struct LoginUserResponseModel {
pub token: String
}


#[derive(Serialize,Deserialize)]
pub struct UpdateUser{
pub name: String,
}