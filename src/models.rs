use mysql_async::prelude::FromRow;
use mysql_async::Row;
/*
# SQL define
#
# 1. users Table
#   
#   +----+------+------------+---------------+-----------+
#   | id | name | phone(uni) | open_id(uni ) | stores_id |
#   +----+------+------------+---------------+-----------+
#
# 2. stores Table
#   +----+------+---------+
#   | id | name | manager | 
#   +----+------+---------+
#
# 3. goods Table
#
#   +----+------+-------+----------+-----------+------+------+----------+----------+
#   | id | type | brand | price_in | price_out | desc | addr | pic_addr | fineness |
#   +----+------+-------+----------+-----------+------+------+----------+----------+
#
# 4. orders Table
#
#   +----+----------+----------+-----------+-----------+
#   | id | users_id | goods_id | deal_time | stores_id | 
#   +----+----------+----------+-----------+-----------+
*/


// 1. users Table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub open_id: String,
    pub stores_id: i32,
}

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let (id, name, phone, open_id, stores_id) = mysql_async::from_row(row);
        User {
            id,
            name,
            phone,
            open_id,
            stores_id,
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
           let (id, name, phone, open_id, stores_id) = mysql_async::from_row_opt(row)?;
           Ok(User {
               id,
               name,
               phone,
               open_id,
               stores_id,
           })
    }
    
}


// 2. stores Table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub manager: i32,
}

impl FromRow for Store {
    fn from_row(row: Row) -> Self {
        let (id, name,manager) = mysql_async::from_row(row);
        Store {
            id,
            name,
            manager,
        }
    }
    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
           let (id, name,manager) = mysql_async::from_row_opt(row)?;
           Ok(Store {
               id,
               name,
               manager,
           })
    }
}

// 3. goods Table
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
}

impl FromRow for Good {
    fn from_row(row: Row) -> Self {
        let (id, type_, brand, price_in, price_out, desc, addr, pic_addr, fineness) = mysql_async::from_row(row);
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
        }
    }
    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
           let (id, type_, brand, price_in, price_out, desc, addr, pic_addr, fineness) = mysql_async::from_row_opt(row)?;
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
           })
    }
}

// 4. orders Table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Orders {
    pub id: i32,
    pub users_id: i32,
    pub goods_id: i32,
    pub deal_time: String,
    pub stores_id: i32,
}

impl FromRow for Orders {
    fn from_row(row: Row) -> Self {
        let (id, users_id, goods_id, deal_time, stores_id) = mysql_async::from_row(row);
        Orders {
            id,
            users_id,
            goods_id,
            deal_time,
            stores_id,
        }
    }
    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
           let (id, users_id, goods_id, deal_time, stores_id) = mysql_async::from_row_opt(row)?;
           Ok(Orders {
               id,
               users_id,
               goods_id,
               deal_time,
               stores_id,
           })
    }
}

