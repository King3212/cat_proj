use actix_web::http::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::models::{good, jwt::*};
use crate::handlers::{auth_handler, good_handler};
use crate::models::wx::WxLoginRequest;


#[post("/wx_login")]
pub async fn wx_login_routes(info: web::Json<WxLoginRequest>) -> impl Responder {
    let res = auth_handler::wx_login(&info).await;
    return HttpResponse::Ok().body(res);
    
}
use crate::models::user;
#[post("/passwd_login1")]
pub async fn passwd_login_1_routes(info: web::Json<auth_handler::PasswdLoginRequest1>) -> impl Responder {
    let res = auth_handler::need_passwd_login_1(&info).await;
    return HttpResponse::Ok().json(res);
}

#[post("/passwd_login2")]
pub async fn passwd_login_2_routes(info: web::Json<auth_handler::PasswdLoginRequest2>) -> impl Responder {
    let res = auth_handler::need_passwd_login_2(&info).await;
    return HttpResponse::Ok().json(res);
}

#[post("getGoods")]
pub async fn get_goods_routes(info: web::Json<good_handler::SearchCriteria>) -> impl Responder {
    let res = good_handler::search_goods(&info).await;
    return HttpResponse::Ok().json(res);
}

