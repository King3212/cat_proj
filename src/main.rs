mod test;
mod db;
mod models;

use dotenv::dotenv;
use std::env;
use once_cell::sync::Lazy;
use actix_web::{
    get,App,HttpServer,Responder
};

static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
});


#[get("/")]
async fn greet() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await

} 