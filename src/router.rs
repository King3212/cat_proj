use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize)]
struct WxLoginRequest {
    code: String,
}

#[derive(Deserialize)]
struct WxSessionResponse {
    openid: String,
    session_key: String,
    unionid: Option<String>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}



#[post("/wx_login")]
async fn wx_login(info: web::Json<WxLoginRequest>) -> impl Responder {
    let appid = "your_appid";
    let secret = "your_secret";
    let code = &info.code;
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", appid, secret, code);

    let client = Client::new();
    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let wx_session: WxSessionResponse = response.json().await.unwrap();
            if let Some(errcode) = wx_session.errcode {
                HttpResponse::BadRequest().body(format!("WeChat API error: {}", errcode))
            } else {
                // 在这里处理 openid 和 session_key，例如查找或创建用户信息
                HttpResponse::Ok().body(format!("Login successful, openid: {}", wx_session.openid))
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to call WeChat API"),
    }
}

