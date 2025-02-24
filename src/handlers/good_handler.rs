use serde::Serialize;
use serde_derive::Deserialize;

use crate::models::jwt;

#[derive(Deserialize,Serialize)]
pub struct SearchCriteria {
    pub type_: String,
    pub brand: Vec<String>,
    pub min_price: f64,
    pub max_price: f64,
    pub store_id: i32,
    pub desc: String,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
    pub fineness: Vec<String>,
    pub jwt: String,
}

#[derive(Serialize)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub total: i64,         // 总记录数
    pub page: i32,          // 当前页码
    pub page_size: i32,     // 每页大小
    pub total_pages: i32,   // 总页数
}

pub struct SearchGood{
    pub type_: String,
    pub brand: String,
    pub desc: String,
    pub pic_urls: Vec<String>,
    pub price_in: f64,
    pub addr: String,

}

pub struct SearchResult {
    pub data: PaginatedData<SearchGood>,
    pub status: i32,
    pub message: String,
}

pub async fn search_goods(criteria: &SearchCriteria)-> SearchResult {
    let result = jwt::validate_jwt(&criteria.jwt);
    if result.is_err() {
        return SearchResult {
            data: PaginatedData {
                items: vec![],
                total: 0,
                page: 0,
                page_size: 0,
                total_pages: 0,
            },
            status: 401,
            message: "Unauthorized".to_string(),
        };
    }else{
        
    }
}

