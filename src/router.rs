use actix_web::{cookie::{time::Duration, Expiration}, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use reqwest::Client;

use crate::{db::{get_user_by_open_id, create_user_by_open_id, update_user_by_open_id}, models::User, SECRET_KEY, WECHAT_APPID, WECHAT_SECRET, POOL};

#[derive(Deserialize)]
struct WxLoginRequest {
    code: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}


#[derive(Deserialize)]
struct WxSessionResponse {
    openid: String,
    session_key: String,
    unionid: Option<String>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}

fn generate_jwt(openid: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(TimeDelta::seconds(3600))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: openid.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret((SECRET_KEY).as_ref())).unwrap()
}


// login 
#[post("/wx_login")]
async fn wx_login(info: web::Json<WxLoginRequest>) -> impl Responder {
    // for test
    let token = generate_jwt("test");
    return HttpResponse::Ok().json(token);

    let appid = WECHAT_APPID.as_str();
    let secret = WECHAT_SECRET.as_str();
    let code = &info.code;
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", appid, secret, code);

    let client = Client::new();
    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let wx_session: WxSessionResponse = response.json().await.unwrap();
            if let Some(errcode) = wx_session.errcode {
                return HttpResponse::BadRequest().body(format!("WeChat API error: {}", errcode));
            } 
            match get_user_by_open_id(&POOL,&wx_session.openid).await {
                Ok(_user) => {
                    // user 是查询到的用户数据
                    // 此处表示用户已存在
                    let token = generate_jwt(&wx_session.openid);
                    HttpResponse::Ok().json(token)
                }
                Err(_e) => {
                    // 查询失败或用户不存在
                    // 创建用户
                    match create_user_by_open_id(&POOL,&wx_session.openid).await {
                        Ok(_ok)=>{}
                        Err(_err)=>{}
                    }
                    let token = generate_jwt(&wx_session.openid);
                    HttpResponse::Ok().json(token)

                }
            }


        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to call WeChat API"),
    }
}


#[derive(Deserialize)]
struct CreateAccountInfo {
    token: String,
    username: String,
    password: String,
    phone: String,
    email: String,
}


// create user
#[post("/create_user")]
async fn create_user(info: web::Json<CreateAccountInfo>) -> impl Responder {
    let user = info.into_inner();
    let result = update_user_by_open_id(&POOL, &user.username, &user.password, &user.phone, &user.email).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}


#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}