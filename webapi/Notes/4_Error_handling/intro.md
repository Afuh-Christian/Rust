# Expect .. 

- 

```rs 
    let db: DatabaseConnection = Database::connect(database_url).await.expect("Database connection Failed");
    Migrator::up(&db, None).await.expect("Migration Failed");
```


# To get custom results .. use a `struct`

```rs

// error 
struct MainError{
   message : String 
}

// ? = checks for error and returns the error if . 

async fn main () -> Result<(),MainError> {


operation.map_err(|err |MainError{message : err.to_string()})?; // when the execution continues . 

operation2.map_err(|err |MainError{message : err.to_string()}); // when you want to return ..

}

```

## Apply on the constants .. suppose it doesn't pull from the .env


```rs
use std::env;

use lazy_static::lazy_static;



lazy_static!{

    pub static ref ADDRESS: String = set_address();
    pub static ref SECRET: String = set_secret();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();

}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or("localhost".to_string())
}
fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap_or("SECRET".to_string())
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("database url string")
}


fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap_or("8080".to_owned()).parse::<u16>().unwrap()
}
```


## Implementing trait for the error .. 


```rs 


// error 
#[derive(Debug)]
struct MainError{
    message : String 
 }



 impl Error for MainError {
        fn description(&self) -> &str {
        "description() is deprecated; use Display"
        }
     
        fn cause(&self) -> Option<&dyn Error> {
        self.source()
        }
              
        fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
        }
    }
 
 impl Display for MainError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
 

```

## Applying on the handlers ... 


- 
```rs 

// impl response changed to Result<ApiResponse,ApiResponse>

    #[post("/register")]
    pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>
    ) -> Result<ApiResponse, ApiResponse> {

    let user_model: Model = entity::user::ActiveModel {
    name: Set(register_json.name.clone()),
    email: Set(register_json.email.clone()),
    password: Set(digest(&register_json.password)),
    ..Default::default()
    }.insert(&app_state.db).await
    //
    .map_err(|err| api_response::ApiResponse::new(500, format!("{}", err)))?;
    //
    Ok(api_response::ApiResponse::new(200, format!("{}",user_model.id)))
    }
    





#[post("/login")]
pub async fn login(
app_state: web:: Data<app_state:: AppState>,
login_json: web:: Json<LoginModel>
) -> Result<ApiResponse, ApiResponse> {

let user_data = entity::user:: Entity::find()
.filter(
Condition::all()
.add(entity::user::Column:: Email.eq(&login_json.email))
.add(entity::user::Column:: Password.eq(digest(&login_json.password)))
).one(&app_state.db).await
//
.map_err(|err | ApiResponse::new(500,err.to_string()))?
.ok_or(ApiResponse::new(404, "User Not Found".to_owned()))?;
//

let token: String = encode_jwt(user_data.email, user_data.id)
//
.map_err(|err | ApiResponse::new(500,err.to_string()))?;
//
Ok(api_response::ApiResponse::new(200, format! ("{{ 'token':'{}' }}", token)))

}

    



```