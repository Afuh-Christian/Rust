use actix_web::{ middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

mod utils;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

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

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    
    HttpServer::new(move || {
        App::new()
        // add middleware
        .app_data(web::Data::new(AppState{db: db.clone()}))
            .wrap(Logger::default())
        // add middleware
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}