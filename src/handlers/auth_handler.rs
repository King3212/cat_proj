use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::jwt::*;
use crate::models::wx::*;
use crate::handlers::user_handler::*;
use actix_web::http::Error;
use std::result;

// 检查是否合规，返回jwt或错误
pub async fn wx_login_check(info: &web::Json<WxLoginRequest>) -> String {
    // 向服务器请求code对应的数据
    let res = wx_login(&info.code).await; 
    // 检查res是否为error
    if res.is_err() {
        return "Error, please check your code".to_string();
    }
    // 解析res
    let wx_session: WxLoginResponse = res.unwrap();
    // 要求数据库添加用户
    add_user_to_db_by_openid(&wx_session.open_id).await;
    // 根据返回的数据生成token
    let token = generate_jwt(&wx_session.open_id);
    // 返回token    
    return token;
}




// // login 
// #[post("/wx_login")]
// async fn wx_login(info: web::Json<WxLoginRequest>) -> impl Responder {
//     // for test
//     let token = generate_jwt("test");
//     return HttpResponse::Ok().json(token);

//     let appid = WECHAT_APPID.as_str();
//     let secret = WECHAT_SECRET.as_str();
//     let code = &info.code;
//     let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", appid, secret, code);

//     let client = Client::new();
//     let res = client.get(&url).send().await;

//     match res {
//         Ok(response) => {
//             let wx_session: WxSessionResponse = response.json().await.unwrap();
//             if let Some(errcode) = wx_session.errcode {
//                 return HttpResponse::BadRequest().body(format!("WeChat API error: {}", errcode));
//             } 
//             match get_user_by_open_id(&POOL,&wx_session.openid).await {
//                 Ok(_user) => {
//                     // user 是查询到的用户数据
//                     // 此处表示用户已存在
//                     let token = generate_jwt(&wx_session.openid);
//                     HttpResponse::Ok().json(token)
//                 }
//                 Err(_e) => {
//                     // 查询失败或用户不存在
//                     // 创建用户
//                     match create_user_by_open_id(&POOL,&wx_session.openid).await {
//                         Ok(_ok)=>{}
//                         Err(_err)=>{}
//                     }
//                     let token = generate_jwt(&wx_session.openid);
//                     HttpResponse::Ok().json(token)

//                 }
//             }


//         }
//         Err(_) => HttpResponse::InternalServerError().body("Failed to call WeChat API"),
//     }
// }
