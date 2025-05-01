use actix_web::{cookie::time::util, get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};

mod utils;
mod routes;


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



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



    HttpServer::new(|| {
        App::new()
        // add middleware
            .wrap(Logger::default())
        // add middleware
            // .service(hello)
            .configure(routes::home_routes::config)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind((address, port))?
    .run()
    .await
}