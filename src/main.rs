mod test;
mod db;
mod models;
mod router;
use dotenv::dotenv;
use router::wx_login;
use std::env;
use once_cell::sync::Lazy;
use actix_web::{
    get,App,HttpServer,Responder
};

static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
});


#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(wx_login)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await

} 