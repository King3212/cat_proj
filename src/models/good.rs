use std::result;

use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::{Row, Serialized};


/*
#   goods Table
#
#   +----+------+-------+----------+-----------+------+------+----------+----------+
#   | id | type | brand | price_in | price_out | desc | addr | pic_addr | fineness |
#   +----+------+-------+----------+-----------+------+------+----------+----------+
#
*/


//  goods Table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Good {
    pub id: i32,
    pub type_: String,
    pub brand: String,
    pub price_in: f64,
    pub price_out: f64,
    pub desc: String,
    pub addr: String,
    pub pic_addr: String,
    pub fineness: String,
    pub stores_id: i32,
}

impl FromRow for Good {
    fn from_row(row: Row) -> Self {
        let (id, type_, brand, price_in, price_out, desc, addr, pic_addr, fineness,store_id) = mysql_async::from_row(row);
        Good {
            id,
            type_,
            brand,
            price_in,
            price_out,
            desc,
            addr,
            pic_addr,
            fineness,
            stores_id: store_id,
        }
    }
    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
           let (id, type_, brand, price_in, price_out, desc, addr, pic_addr, fineness,store_id) = mysql_async::from_row_opt(row)?;
           Ok(Good {
               id,
               type_,
               brand,
               price_in,
               price_out,
               desc,
               addr,
               pic_addr,
               fineness,
               stores_id: store_id,
           })
    }
}




use mysql_async::{Pool,params};
use crate::handlers::good_handler;
use crate::models::jwt;
use crate::models::user;
use mysql_async::*;
use mysql_async::Error;
use mysql_async::prelude::*;
use std::default;
use std::result::Result;


pub async fn search_goods_nums(POOL: &Pool, searchCriteria: &good_handler::SearchCriteria) -> i64 {
    let mut conn = POOL.get_conn().await.unwrap();
    let open_id = jwt::get_openid_from_jwt(&searchCriteria.jwt);
    let store_id = user::get_user_by_open_id(POOL, &open_id.unwrap()).await.unwrap().unwrap().stores_id;
    let mut pre_query = "SELECT COUNT(*) FROM goods WHERE store_id = ?".to_string();
    let mut args: Vec<String> = Vec::new();
    args.push(store_id.to_string());

    // 类型过滤
    if !searchCriteria.type_.is_empty() {
        pre_query.push_str(" AND type = ?");
        args.push(searchCriteria.type_.clone());
    }

    // 品牌过滤
    if !searchCriteria.brand.is_empty() {
        pre_query.push_str(" AND (");
        for (i, brand) in searchCriteria.brand.iter().enumerate() {
            if i > 0 {
                pre_query.push_str(" OR ");
            }
            pre_query.push_str("brand = ?");
            args.push(brand.clone());
        }
        pre_query.push_str(")");
    }

    // 精度过滤
    if !searchCriteria.fineness.is_empty() {
        pre_query.push_str(" AND (");
        for (i, fineness) in searchCriteria.fineness.iter().enumerate() {
            if i > 0 {
                pre_query.push_str(" OR ");
            }
            pre_query.push_str("fineness = ?");
            args.push(fineness.clone());
        }
        pre_query.push_str(")");
    }

    // 价格过滤
    if searchCriteria.max_price > 0.0 {
        pre_query.push_str(" AND price_in <= ?");
        args.push(searchCriteria.max_price.to_string());
    }
    if searchCriteria.min_price > 0.0 && searchCriteria.min_price < searchCriteria.max_price {
        pre_query.push_str(" AND price_in >= ?");
        args.push(searchCriteria.min_price.to_string());
    }

    // 描述过滤
    if !searchCriteria.desc.is_empty() {
        pre_query.push_str(" AND desc LIKE ?");
        args.push(format!("%{}%", searchCriteria.desc));
    }

    let total_count: i64 = conn.exec_first(pre_query.as_str(), args.clone()).await.unwrap_or(Some(0)).unwrap();
    total_count
}

pub async fn search_goods(POOL: &Pool, searchCriteria: &good_handler::SearchCriteria) -> Vec<good_handler::SearchGood> {
    let mut conn = POOL.get_conn().await.unwrap();
    let open_id = jwt::get_openid_from_jwt(&searchCriteria.jwt);
    let store_id = user::get_user_by_open_id(POOL, &open_id.unwrap()).await.unwrap().unwrap().stores_id;
    let mut pre_query = "SELECT * FROM goods WHERE store_id = ?".to_string();
    let mut args: Vec<String> = Vec::new();
    args.push(store_id.to_string());

    // 类型过滤
    if !searchCriteria.type_.is_empty() {
        pre_query.push_str(" AND type = ?");
        args.push(searchCriteria.type_.clone());
    }

    // 品牌过滤
    if !searchCriteria.brand.is_empty() {
        pre_query.push_str(" AND (");
        for (i, brand) in searchCriteria.brand.iter().enumerate() {
            if i > 0 {
                pre_query.push_str(" OR ");
            }
            pre_query.push_str("brand = ?");
            args.push(brand.clone());
        }
        pre_query.push_str(")");
    }

    // 精度过滤
    if !searchCriteria.fineness.is_empty() {
        pre_query.push_str(" AND (");
        for (i, fineness) in searchCriteria.fineness.iter().enumerate() {
            if i > 0 {
                pre_query.push_str(" OR ");
            }
            pre_query.push_str("fineness = ?");
            args.push(fineness.clone());
        }
        pre_query.push_str(")");
    }

    // 价格过滤
    if searchCriteria.max_price > 0.0 {
        pre_query.push_str(" AND price_in <= ?");
        args.push(searchCriteria.max_price.to_string());
    }
    if searchCriteria.min_price > 0.0 && searchCriteria.min_price < searchCriteria.max_price {
        pre_query.push_str(" AND price_in >= ?");
        args.push(searchCriteria.min_price.to_string());
    }

    // 描述过滤
    if !searchCriteria.desc.is_empty() {
        pre_query.push_str(" AND desc LIKE ?");
        args.push(format!("%{}%", searchCriteria.desc));
    }

    // 排序字段
    let sort_by = &searchCriteria.sort_by;
    if !sort_by.is_empty() {
        pre_query.push_str(" ORDER BY ");
        pre_query.push_str(sort_by);
        pre_query.push_str(" ASC");
    }

    // 分页
    pre_query.push_str(" LIMIT ? OFFSET ?");
    args.push(searchCriteria.page_size.to_string());
    let num = (searchCriteria.page - 1) * searchCriteria.page_size;
    args.push(num.to_string());

    // 执行查询并处理结果
    let query = pre_query;
    let result: Vec<good_handler::SearchGood> = conn.exec_map(
        query.as_str(),
        args,
        |(id, type_, brand, fineness, price_in, description, addr, pic_urls)| {
            good_handler::SearchGood {
                id,
                type_,
                brand,
                fineness,
                price_in,
                description,
                addr,
                pic_urls,
            }
        }
        
    ).await.unwrap();  // 这里你可以选择更精确的错误处理

    result
}


