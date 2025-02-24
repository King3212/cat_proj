use mysql_async::prelude::FromRow;
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
    pub store_id: i32,
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
            store_id,
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
               store_id,
           })
    }
}




use mysql_async::{Pool};
use crate::handlers::good_handler;
use crate::models::jwt;
use crate::models::user;






pub async fn search_goods(POOL: &Pool ,searchCriteria: &good_handler::SearchCriteria) -> Vec<good_handler::SearchGood>{
    let mut conn = POOL.get_conn().await.unwrap();
    let open_id = jwt::get_openid_from_jwt(&searchCriteria.jwt);
    let store_id = user::get_user_by_open_id(POOL, &open_id.unwrap()).await.unwrap().unwrap().stores_id;
    let mut pre_query = "SELECT * FROM goods WHERE store_id = ?".to_string();
    let mut args: Vec<String> = Vec::new();
    args.push(store_id.to_string());
    
    if searchCriteria.type_ != "".to_string() {
        pre_query.push_str(" AND type = ?");
        args.push(searchCriteria.type_.clone());
    }
    if !searchCriteria.brand.is_empty(){
        pre_query.push_str(" AND brand = ?");
        args.push(searchCriteria.brand.get(0).unwrap().to_string());

        for i in 1..searchCriteria.brand.len(){
            pre_query.push_str(" OR brand = ?");
            args.push(searchCriteria.brand.get(i).unwrap().to_string());
        }
    }
    if !searchCriteria.fineness.is_empty(){
        pre_query.push_str(" AND fineness = ?");
        args.push(searchCriteria.fineness.get(0).unwrap().to_string());

        for i in 1..searchCriteria.fineness.len(){
            pre_query.push_str(" OR fineness = ?");
            args.push(searchCriteria.fineness.get(i).unwrap().to_string());
        }
    }
    
    if searchCriteria.max_price > 0.0 {
        pre_query.push_str(" AND price_in <= ?");
        args.push(searchCriteria.max_price.to_string());
    }
    if searchCriteria.min_price > 0.0 && searchCriteria.min_price < searchCriteria.max_price {
        pre_query.push_str(" AND price_in >= ?");
        args.push(searchCriteria.min_price.to_string());
    }
    
    if searchCriteria.desc != "".to_string() {
        pre_query.push_str(" AND desc LIKE ?");
        args.push(format!("%{}%", searchCriteria.desc));
    }

    if !searchCriteria.fineness.is_empty() {
        pre_query.push_str(" AND fineness = ?");
        args.push(searchCriteria.fineness.get(0).unwrap().to_string());

        for i in 1..searchCriteria.fineness.len(){
            pre_query.push_str(" OR fineness = ?");
            args.push(searchCriteria.fineness.get(i).unwrap().to_string());
        }
    }

    pre_query.push_str(" ORDER BY ? ASC");
    args.push(searchCriteria.sort_by.to_string());

    pre_query.push("LIMIT ? OFFSET ?");
    args.push(searchCriteria.page_size.to_string());
    let num = (searchCriteria.page - 1) * searchCriteria.page_size;
    args.push(num.to_string());

    let query = pre_query.to_string();
    let mut result = Vec::new();

    let mut stmt = conn.prepare(&query).unwrap();
    let mut rows = stmt.query(args).unwrap();
    while let Some(row) = rows.next().unwrap() {
        
        

    return result;
}

