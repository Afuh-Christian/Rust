use chrono::{DateTime, NaiveDateTime};
use derive_more::From;
use entity::post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize, Clone , From )] // for viewing data ... allows cloning
pub struct PostModel{
    pub id: i32,
    pub title: String,
    pub text: String,
    pub image: String,
    pub user_id: i32,
    pub uuid: Uuid , 
    pub created_at: NaiveDateTime,
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
        }
    }
}


#[derive(Serialize,Deserialize)]
pub struct CreatePostModel{
    pub title: String,
    pub text: String,
    pub image: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

