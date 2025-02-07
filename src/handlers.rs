use axum::{extract::Path, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

// 获取所有商品
pub async fn get_items() -> Json<Vec<Item>> {
    let items = vec![Item { id: 1, name: "二手电器".to_string() }];
    Json(items)
}

// 获取单个商品
pub async fn get_item(Path(id): Path<u32>) -> Json<Item> {
    Json(Item { id, name: format!("商品 {}", id) })
}

// 登录示例
#[derive(Deserialize)]
pub struct LoginRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    Json(LoginResponse { token: format!("token_for_{}", payload.code) })
}
