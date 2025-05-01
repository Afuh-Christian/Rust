use std::{error::Error, fmt::Display};

use actix_web::{ middleware::Logger, web, App, HttpServer, Result};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

mod utils;
mod routes;


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
 
 


#[actix_web::main]
async fn main() -> Result<(),MainError> {

    // app loggers for requets and responses
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    // env logger 
    dotenv::dotenv().ok();
    env_logger::init();
    // start the server


    let port: u16 = (*utils::constants::PORT).clone();
    let address: String = (*utils::constants::ADDRESS).clone();
    let database_url: String = (*utils::constants::DATABASE_URL).clone();

    println!("Port Number : {} " , port);


    let db: DatabaseConnection = Database::connect(database_url).await.map_err(|err |MainError{message : err.to_string()})?;
    Migrator::up(&db, None).await.map_err(|err |MainError{message : err.to_string()})?;

    
    HttpServer::new(move || {
        App::new()
        // add middleware
        .app_data(web::Data::new(AppState{db: db.clone()}))
            .wrap(Logger::default())
        // add middleware
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
    })
    .bind((address, port))
    .map_err(|err |MainError{message : err.to_string()})?
    .run()
    .await
    .map_err(|err |MainError{message : err.to_string()})
}