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

pub async fn wx_login(code: &String, base_url: Option<&str>) -> Result<WxLoginResponse, reqwest::Error> {
    let api_base = base_url.unwrap_or("https://api.weixin.qq.com");
    let url = format!(
        "{}/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
        api_base,
        WECHAT_APPID.as_str(),
        WECHAT_SECRET.as_str(),
        code
    );
    reqwest::get(&url).await?.json().await
}
pub async fn wx_login_api_for_test(code: &String) -> Result<WxLoginResponse, reqwest::Error> {
    let response: WxLoginResponse = WxLoginResponse {
        open_id: "123".to_string(),
        session_key: "123".to_string(),
        union_id: None,
        errcode: None,
        errmsg: None,
    };
    Ok(response)
}