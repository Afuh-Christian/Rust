use axum::{http::{header, StatusCode}, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError{
    pub message:String, 
    pub error_code:Option<i8>,
    pub status_code:StatusCode
}

impl IntoResponse for ApiError {
    

    fn into_response(self) -> axum::response::Response {
        
        // let status_code = self.status_code;
        (
            self.status_code.clone() , 
            [(header::CONTENT_TYPE,"application/json")], 
            Json(json!({"StatusCode":self.status_code.as_u16() , "ErrorCode":self.error_code , "Message" : self.message}))
    ).into_response()
    }
}