use mysql_async::prelude::FromRow;
use mysql_async::Row;

/*
    User table
    +----+--------+----------+----------+------------+---------+
    | id | name   | password | store_id | phone      | wx_id   |
    +----+--------+----------+----------+------------+---------+
*/

#[derive(Debug, Clone)]
pub struct Users {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub store_id: String,
    pub phone: String,
    pub wx_id: String,
}

impl FromRow for Users {
    fn from_row(row: Row) -> Self {
        let (id, name, password, store_id, phone, wx_id) = mysql_async::from_row(row);
        Users {
            id: Some(id),
            name,
            password,
            store_id,
            phone,
            wx_id,
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (id, name, password, store_id, phone, wx_id) = mysql_async::from_row_opt(row)?;
        Ok(Users {
            id: Some(id),
            name,
            password,
            store_id,
            phone,
            wx_id,
        })
    }
}

/*
    Goods table
    +----+--------+--------+-------+----------+--------+----------+----------+
    | id | type   | brand  | price | fineness | desc   | location | store_id |
    +----+--------+--------+-------+----------+--------+----------+----------+
    +--------+
    | pic_url|
    +--------+
*/

#[derive(Debug, Clone)]
pub struct Goods {
    pub id: Option<i32>,
    pub type_: String,
    pub brand: String,
    pub price: f32,
    pub fineness: String,
    pub desc: String,
    pub location: String,
    pub store_id: String,
}

impl FromRow for Goods {
    fn from_row(row: Row) -> Self {
        let (id, type_, brand, price, fineness, desc, location,store_id) = mysql_async::from_row(row);
        Goods {
            id: Some(id),
            type_,
            brand,
            price,
            fineness,
            desc,
            location,
            store_id,
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (id, type_, brand, price, fineness, desc, location,store_id) = mysql_async::from_row_opt(row)?;
        Ok(Goods {
            id: Some(id),
            type_,
            brand,
            price,
            fineness,
            desc,
            location,
            store_id,
        })
    }
}

/*
    Orders table
    +----+---------+----------+------------+-----------+--------+
    | id | user_id | goods_id | deal_price | deal_time | status |
    +----+---------+----------+------------+-----------+--------+
*/

#[derive(Debug, Clone)]
pub struct Orders {
    pub id: Option<i32>,
    pub user_id: i32,
    pub goods_id: i32,
    pub deal_price: f32,
    pub deal_time: String,
    pub status: i32,
}

impl FromRow for Orders {
    fn from_row(row: Row) -> Self {
        let (id, user_id, goods_id, deal_price, deal_time, status) = mysql_async::from_row(row);
        Orders {
            id: Some(id),
            user_id,
            goods_id,
            deal_price,
            deal_time,
            status,
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (id, user_id, goods_id, deal_price, deal_time, status) = mysql_async::from_row_opt(row)?;
        Ok(Orders {
            id: Some(id),
            user_id,
            goods_id,
            deal_price,
            deal_time,
            status,
        })
    }
}


/*
    store table
    +-------+------------+---------+
    | id    |  
*/