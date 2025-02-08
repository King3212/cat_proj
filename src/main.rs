mod test;
mod db;
mod models;
mod router;
mod jwt;
use dotenv::dotenv;
use router::wx_login;
use std::env;
use once_cell::sync::Lazy;
use actix_web::{
    get,App,HttpServer,Responder
};


// set env
static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
});
static WECHAT_APPID: Lazy<String> = Lazy::new(||{
    dotenv().ok();
    env::var("WECHAT_APPID").expect("WECHAT_APPID must be set in .env file")
});
static WECHAT_SECRET: Lazy<String> = Lazy::new(||{
    dotenv().ok();
    env::var("WECHAT_SECRET").expect("WECHAT_SECRET must be set in .env file")
});
static SECRET_KEY: Lazy<String> = Lazy::new(||{
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file")
});


#[actix_web::main]
async fn main()-> std::io::Result<()> {
    println!("localhost:3000");
    HttpServer::new(|| {
        App::new()
            .service(wx_login)
    })
    .bind("localhost:3000")?
    .run()
    .await

} 