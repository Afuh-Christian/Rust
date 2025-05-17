use chrono::{ NaiveDateTime};
use entity::post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user_model::UserMicroModel;

#[derive(Serialize,Deserialize , Default)] // for viewing data ... allows cloning
pub struct PostModel{
    pub id: i32,
    pub title: String,
    pub text: String,
    pub image: String,
    pub user_id: i32,
    pub uuid: Uuid , 
    pub created_at: NaiveDateTime,
    pub user : Option<UserMicroModel>
}


impl From<(entity::post::Model, Option<entity::user::Model>)> for PostModel {
 fn from(value : (entity::post::Model, Option<entity::user::Model>)) -> Self {
    let u = value.1.unwrap();
    return PostModel{
        uuid : value.0.uuid,
        text : value.0.text,
        created_at : value.0.created_at,
        title : value.0.title,
        id : value.0.id,
        image : value.0.image,
        user_id : value.0.user_id,  
        user : Some(UserMicroModel { name: u.name, uuid:u.uuid })  // user info
        ,
        ..Default::default() 
    };
 }
}

impl PostModel {
   pub fn new(model: post::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            text: model.text,
            image: model.image,
            uuid: model.uuid,
            user_id: model.user_id,
            created_at: model.created_at,
            user:None
        }
    }
}


#[derive(Serialize,Deserialize)]
pub struct CreatePostModel{
    pub title: String,
    pub text: String,
    pub image: String
}

