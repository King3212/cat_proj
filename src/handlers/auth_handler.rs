use actix_web::{web,};

use crate::models::jwt::{self, *};
use crate::models::wx;
use crate::handlers::user_handler::*;
use serde::Serialize;
use super::user_handler;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct PasswdLoginRequest1 {
    pub phone: String,
}

#[derive(Deserialize, Serialize)]
pub struct PasswdLoginResponse1 {
    pub salt: String,
    pub random_code: String,
    pub code: String,
    pub msg: String,
}

#[derive(Deserialize)]
pub struct PasswdLoginRequest2 {
    pub phone: String,
    pub passwd_salted: String
}


#[derive(Deserialize, Serialize)]
pub struct PasswdLoginResponse2 {
    pub code: String,
    pub jwt: String,
}





// 检查是否合规，返回jwt或错误
pub async fn wx_login(info: &web::Json<wx::WxLoginRequest>) -> String {
    // 向服务器请求code对应的数据
    let res = wx::wx_login(&info.code,None).await; 
    // 检查res是否为error
    if res.is_err() {
        return "Error, please check your code".to_string();
    }
    // 解析res
    let wx_session: wx::WxLoginResponse = res.unwrap();
    // 要求数据库添加用户
    add_user_to_db_by_openid(&wx_session.open_id).await;
    // 根据返回的数据生成token
    let token = generate_jwt(&wx_session.open_id);
    // 返回token    
    return token;
}

use rand_core::{TryRngCore, OsRng};

pub async fn need_passwd_login_1(info: &web::Json<PasswdLoginRequest1>) -> PasswdLoginResponse1 {
    let user: crate::models::user::User = user_handler::get_user_by_phone(&info.phone).await;
    if user.id == 0 {
        return PasswdLoginResponse1 {
            code: "100".to_string(),
            msg: "用户不存在".to_string(),
            salt: "".to_string(),
            random_code: "".to_string(),
        };
    }else{
        let mut rng: OsRng = OsRng;
        let mut key: [u8; 6] = [0u8; 6];
        rng.try_fill_bytes(&mut key).unwrap();
        let key: String = hex::encode(key);
        return PasswdLoginResponse1 {
            code: "200".to_string(),
            msg: "用户存在".to_string(),
            salt: user.salt,
            random_code: key,
        };

    }
    
}

pub async fn need_passwd_login_2(info: &web::Json<PasswdLoginRequest2>) -> PasswdLoginResponse2 {
    let user = user_handler::get_user_by_phone(&info.phone).await;
    if user.id == 0 {
        return PasswdLoginResponse2 {
            code: "100".to_string(),
            jwt: "".to_string(),
        };
    }else{
        if user.local_hash != info.passwd_salted {
            return PasswdLoginResponse2 {
                code: "101".to_string(),
                jwt: "".to_string(),
            };
        }
        let token = generate_jwt(&user.id.to_string());
        return PasswdLoginResponse2 {
            code: "200".to_string(),
            jwt: token
        };
    }
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
