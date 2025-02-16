use dotenv::dotenv;
use std::env;
use once_cell::sync::Lazy;
use actix_web::{
    get,App,HttpServer,Responder,HttpResponse,web
};

mod handlers;
mod models;
mod routes;
use routes::api_routes;


// set env
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

// set pool
static POOL: Lazy<mysql_async::Pool> = Lazy::new(||{
    dotenv().ok();
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    mysql_async::Pool::new(DATABASE_URL.as_str())
});

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(wx_login_routes)
        .service(create_user); // 需要确保create_user路由存在
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    println!("localhost:3000");
    HttpServer::new(|| {
        App::new()
        .configure(api_routes::config)
    })
    .bind("localhost:3000")?
    .run()
    .await

} 