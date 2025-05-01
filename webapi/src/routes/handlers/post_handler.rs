use actix_web::{post,get, web};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ ActiveValue::{Set}};
use serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use uuid::Uuid;
use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use crate::utils::{api_response::{self, ApiResponse}, app_state, jwt::Claims};

#[derive(Serialize, Deserialize)]
struct CreatePostModel {
    title: String,
    text: String
    }

    
    #[derive(Serialize,Deserialize)]
    struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image:Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub user: Option<UserModel>,
    }




    #[derive(Serialize, Deserialize)]
    struct UserModel {
        pub name: String, 
        pub email: String,
    }

















#[post("create")]
pub async fn create_posts(
    app_state: web::Data<app_state::AppState>,
    claims_data: Claims,
    post_model: web::Json<CreatePostModel>,
) -> Result<ApiResponse, ApiResponse>{
    let post_entity = entity::post::ActiveModel
    {
    title: Set(post_model.title.clone()),
    text: Set(post_model.text.clone()),
    uuid: Set(Uuid::new_v4()),
    user_id: Set(claims_data.id),
    created_at:Set(Utc::now().naive_local()),
    image: Set("".to_string()), // Assuming you have a field for image in your model
    // created_at: Set(chrono::Utc::now().naive_utc()), // Assuming you have a field for created_at in your model
    ..Default::default()
    };
    post_entity.insert(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;  


    Ok(api_response::ApiResponse::new(201 , "Post created successfully".to_owned()))
}













#[get("my-posts")]  
pub async fn my_posts(  
app_state: web::Data<app_state:: AppState>,  
claim: Claims  
)-> Result<api_response:: ApiResponse,api_response:: ApiResponse> {  
let posts: Vec<PostModel> = entity:: post:: Entity:: find()  
.filter(entity::post::Column:: UserId.eq(claim.id))
.all(&app_state.db)  
.await  
.map_err(|err| api_response::ApiResponse::new(500, err.to_string()))? 
.into_iter()  
.map(|post|  
     PostModel {  
         id: post.id,  
         title: post.title,  
         text: post.text,  
         uuid: post.uuid,  
         image: Some(post.image),  
         user_id: post.user_id,
         created_at: post.created_at,
         user: None 
     }  
).collect();  
let res_str: String = serde_json::to_string(&posts)  
.map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;  

Ok(api_response::ApiResponse::new(200, res_str.to_owned())) 
}


















#[get("all-posts")]  
pub async fn all_posts(  
    app_state: web::Data<app_state::AppState>,  
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {  
    let posts: Vec<PostModel> = entity::post::Entity::find()  
        .all(&app_state.db)  
        .await  
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))? 
        .into_iter()  
        .map(|post|  
             PostModel {  
                 id: post.id,  
                 title: post.title,  
                 text: post.text,  
                 uuid: post.uuid,  
                 image: Some(post.image),  
                 user_id: post.user_id,
                 created_at: post.created_at  ,
                 user: None
             }  
        ).collect();  
    let res_str: String = serde_json::to_string(&posts)  
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;  
  
    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))  
}  




















#[get("post/{post_uuid}")]  
pub async fn one_posts(  
app_state: web::Data<app_state::AppState>,  
post_uuid: web::Path<Uuid>  
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {  
  
let posts: PostModel = entity::post::Entity::find()  
.filter(entity::post::Column::Uuid.eq(post_uuid.clone()))  
.find_also_related(entity::user::Entity) // Left join with user table
.one(&app_state.db).await 
.map_err(|err| api_response::ApiResponse::new(500, err.to_string()))? 
.map(|post|  
PostModel {  
id: post.0.id,  
title: post.0.title,  
text: post.0.text,  
uuid: post.0.uuid,  
image: Some(post.0.image),  
user_id: post.0.user_id,  
created_at: post.0.created_at,
user: post.1.map(|user| UserModel {
name: user.name,
email: user.email,
})

}  
)  
.ok_or(api_response::ApiResponse::new(404, "No Post Found".to_string()))?;  
  
let res_str: String = serde_json::to_string(&posts)  
.map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;  
  
Ok(api_response::ApiResponse::new(200, res_str.to_owned()))  

}