use actix_web::http::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::models::jwt::*;

use crate::handlers::auth_handler::*;
use crate::models::wx::WxLoginRequest;


#[post("/wx_login")]
pub async fn wx_login_routes(info: web::Json<WxLoginRequest>) -> impl Responder {
    let res = wx_login_check(&info).await;
    return HttpResponse::Ok().body(res);
    
}


