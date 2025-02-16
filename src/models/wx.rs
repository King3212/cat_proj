use serde::Deserialize;
use crate::WECHAT_APPID;
use crate::WECHAT_SECRET;

#[derive(Deserialize)]
pub struct WxLoginResponse {
    pub open_id: String,
    pub session_key: String,
    pub union_id: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
}

#[derive(Deserialize)]
pub struct WxLoginRequest {
    pub code: String,
}

pub async fn wx_login(code: &String) -> Result<WxLoginResponse, reqwest::Error> {
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
                      WECHAT_APPID.as_str(), WECHAT_SECRET.as_str(), &code);
    let res = reqwest::get(&url).await?.json::<WxLoginResponse>().await?;
    Ok(res)
}