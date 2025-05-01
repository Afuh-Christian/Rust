## Send claims from middleware to the handlers ... 

```rs 

// Implement fromRequest . in the claims ... 



#[derive(Serialize,Deserialize,Clone)]  
pub struct Claims{  
pub exp: usize,  
pub iat: usize,  
pub email: String,  
pub id: i32  
}  
impl FromRequest for Claims{  
type Error = actix_web::Error;  
type Future = future::Ready<Result<Self,Self::Error>>;  
fn from_request(  
    req: &actix_web::HttpRequest,  
    payload: &mut actix_web::dev::Payload  
) -> std::future::Ready<Result<Claims,actix_web::Error>> {  

    match req.extensions().get::<Claims>() {
        Some(claim) => future::ready(Ok(claim.clone())),
        None => future::ready(Err(actix_web::error::ErrorBadRequest( "Bad Claims")))
    } 
}  
}



// Now include in the middleware ... 


    pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth: Option<&HeaderValue> = req.headers().get( AUTHORIZATION);
    
    if auth.is_none() {
    return Err(Error::from(api_response::ApiResponse::new( 401, "Unauthorised".to_string())));
    }
    
    
    let token: String = auth.unwrap().to_str().unwrap().replace("bearer", "").to_owned();
    let claim: TokenData<jwt::Claims> = jwt::decode_jwt(token).unwrap();
    // ---------------
    req.extensions_mut().insert(claim.claims);
    // ---------------
    next.call(req).await 
    .map_err(|err:  Error| Error::from(ApiResponse::new( 500,err.to_string())))
    }


// Now use in the handlers ... 



#[get("")]
pub async fn user(
 app_state: web:: Data<app_state:: AppState> , 
 claims_data : Claims
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

let user_model = entity::user::Entity::find_by_id(claims_data.id)
.one(&app_state.db).await 
.map_err(|err | api_response:: ApiResponse:: new(500, err.to_string()))?
.ok_or(api_response:: ApiResponse:: new(404, "User not found".to_owned()))?;


Ok(api_response:: ApiResponse:: new(200, format!("{{ 'namej' : {} , 'emailh' : {} }}" , user_model.name, user_model.email)))
}




#[post("update")]  
pub async fn update_user(  
app_state: web:: Data<app_state:: AppState>,  
user_data: web:: Json<UpdateUserModel>,  
claim_data: Claims  
) -> Result<api_response::ApiResponse, api_response:: ApiResponse>  {
let mut user_model = entity:: user:: Entity:: find_by_id(claim_data.id)  
.one(&app_state.db).await 
.map_err(|err| api_response:: ApiResponse:: new (500, err.to_string()))? 
.ok_or(api_response:: ApiResponse:: new (404, "User Not Found".to_owned()))?   
.into_active_model();  
user_model.name = Set(user_data.name.clone());  
user_model.update(&app_state.db).await  
.map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;  
Ok (api_response:: ApiResponse:: new (200, "User Updated".to_owned()))  
}  



```


- 


