use serde::Serialize;
use serde_derive::Deserialize;

use crate::models::jwt;

#[derive(Deserialize,Serialize)]
pub struct SearchCriteria {
    pub type_: String,
    pub brand: Vec<String>,
    pub min_price: f64,
    pub max_price: f64,
    pub stores_id: i32,
    pub desc: String,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
    pub fineness: Vec<String>,
    pub jwt: String,
}

#[derive(Serialize)]
pub struct SearchGood{
    pub id: i32,
    pub fineness: String,
    pub type_: String,
    pub brand: String,
    pub description: String,
    pub pic_urls: String,
    pub price_in: f64,
    pub addr: String,

}


#[derive(Serialize)]
pub struct PaginatedData {
    pub items: Vec<SearchGood>,
    pub total: i64,         // 总记录数
    pub page: i32,          // 当前页码
    pub page_size: i32,     // 每页大小
    pub total_pages: i64,   // 总页数
}



#[derive(Serialize)]
pub struct SearchResult {
    pub data: PaginatedData,
    pub status: i32,
    pub message: String,
}


use crate::models::good;
use crate::POOL;

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
        let res =good::search_goods(&POOL,&criteria).await;
        let num = good::search_goods_nums(&POOL,&criteria).await;
        let result = SearchResult {
            data: PaginatedData{
                items: res,
                total: num,
                page: criteria.page,
                page_size: criteria.page_size,
                total_pages: num / criteria.page_size as i64,
            },
            status: 200,
            message: "success".to_string(),
        };
        return result;
    }
}

