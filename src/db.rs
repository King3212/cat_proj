use mysql::*;
use mysql::prelude::*;
use crate::models::Users;
use crate::models::Goods;
use crate::models::Orders;

/* 
    Users表: tabel users 
    +----+--------+------------------+------------+---------+
    | id | name   | password         | phone      | wx_id   |
    +----+--------+------------------+------------+---------+

    Goods表: tabel goods
    +----+--------+--------+-------+----------+--------+----------+
    | id | type   | brand  | price | fineness | desc   | location |
    +----+--------+--------+-------+----------+--------+----------+

    Orders表: tabel orders
    +----+---------+----------+------------+-----------+--------+
    | id | user_id | goods_id | deal_price | deal_time | status |
    +----+---------+----------+------------+-----------+--------+
*/

// 创建用户

pub fn create_user(name: &str, password: &str, phone: &str, wx_id: &str) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"INSERT INTO users (name, password, phone, wx_id) VALUES (:name, :password, :phone, :wx_id)",
        params! {
            "name" => name,
            "password" => password,
            "phone" => phone,
            "wx_id" => wx_id,
        },
    )?;
    Ok(result)
}

// 创建商品

pub fn create_goods(type_: &str, brand: &str, price: f32, fineness: &str, desc: &str, location: &str) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"INSERT INTO goods (type, brand, price, fineness, desc, location) VALUES (:type, :brand, :price, :fineness, :desc, :location)",
        params! {
            "type" => type_,
            "brand" => brand,
            "price" => price,
            "fineness" => fineness,
            "desc" => desc,
            "location" => location,
        },
    )?;
    Ok(result)
}

// 创建订单

pub fn create_order(user_id: i32, goods_id: i32, deal_price: f32, deal_time: &str, status: &str) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"INSERT INTO orders (user_id, goods_id, deal_price, deal_time, status) VALUES (:user_id, :goods_id, :deal_price, :deal_time, :status)",
        params! {
            "user_id" => user_id,
            "goods_id" => goods_id,
            "deal_price" => deal_price,
            "deal_time" => deal_time,
            "status" => status,
        },
    )?;
    Ok(result)
}

// 查询用户

pub fn query_user(name: &str) -> Result<Users, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_first(
        r"SELECT * FROM users WHERE name = :name",
        params! {
            "name" => name,
        },
    )?;
    match result {
        Some(row) => Ok(Users::from_row(row)),
        None => Err(Error::from(mysql::Error::from(mysql::FromValueError(mysql::Value::NULL)))),
    }
}

// 查询商品

pub fn query_goods(id: i32) -> Result<Goods, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_first(
        r"SELECT * FROM goods WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;
    match result {
        Some(row) => Ok(Goods::from_row(row)),
        None => Err(Error::from(mysql::Error::from(mysql::FromValueError(mysql::Value::NULL)))),
    }
}

// 查询订单

pub fn query_order(id: i32) -> Result<Orders, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_first(
        r"SELECT * FROM orders WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;
    match result {
        Some(row) => Ok(Orders::from_row(row)),
        None => Err(Error::from(mysql::Error::from(mysql::FromValueError(mysql::Value::NULL)))),
    }
}

// 更新用户

pub fn update_user(id: i32, name: &str, password: &str, phone: &str, wx_id: &str) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"UPDATE users SET name = :name, password = :password, phone = :phone, wx_id = :wx_id WHERE id = :id",
        params! {
            "id" => id,
            "name" => name,
            "password" => password,
            "phone" => phone,
            "wx_id" => wx_id,
        },
    )?;
    Ok(result)
}

// 更新商品

pub fn update_goods(id: i32, type_: &str, brand: &str, price: f32, fineness: &str, desc: &str, location: &str) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"UPDATE goods SET type = :type, brand = :brand, price = :price, fineness = :fineness, desc = :desc, location = :location WHERE id = :id",
        params! {
            "id" => id,
            "type" => type_,
            "brand" => brand,
            "price" => price,
            "fineness" => fineness,
            "desc" => desc,
            "location" => location,
        },
    )?;
    Ok(result)
}

// 更新订单

pub fn update_order(id: i32, user_id: i32, goods_id: i32, deal_price: f32, deal_time: &str, status: &str) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"UPDATE orders SET user_id = :user_id, goods_id = :goods_id, deal_price = :deal_price, deal_time = :deal_time, status = :status WHERE id = :id",
        params! {
            "id" => id,
            "user_id" => user_id,
            "goods_id" => goods_id,
            "deal_price" => deal_price,
            "deal_time" => deal_time,
            "status" => status,
        },
    )?;
    Ok(result)
}

// 删除用户

pub fn delete_user(id: i32) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"DELETE FROM users WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(result)
}

// 删除商品

pub fn delete_goods(id: i32) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"DELETE FROM goods WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(result)
}

// 删除订单

pub fn delete_order(id: i32) -> Result<usize, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_drop(
        r"DELETE FROM orders WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(result)
}

// 查询所有用户

pub fn query_all_users() -> Result<Vec<Users>, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_map(
        r"SELECT * FROM users",
        (),
        |(id, name, password, phone, wx_id)| {
            Users {
                id: Some(id),
                name,
                password,
                phone,
                wx_id,
            }
        },
    )?;
    Ok(result)
}

// 查询所有商品

pub fn query_all_goods() -> Result<Vec<Goods>, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_map(
        r"SELECT * FROM goods",
        (),
        |(id, type_, brand, price, fineness, desc, location)| {
            Goods {
                id: Some(id),
                type_,
                brand,
                price,
                fineness,
                desc,
                location,
            }
        },
    )?;
    Ok(result)
}

// 查询所有订单

pub fn query_all_orders() -> Result<Vec<Orders>, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_map(
        r"SELECT * FROM orders",
        (),
        |(id, user_id, goods_id, deal_price, deal_time, status)| {
            Orders {
                id: Some(id),
                user_id,
                goods_id,
                deal_price,
                deal_time,
                status,
            }
        },
    )?;
    Ok(result)
}

// 查询用户订单

pub fn query_user_orders(user_id: i32) -> Result<Vec<Orders>, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_map(
        r"SELECT * FROM orders WHERE user_id = :user_id",
        params! {
            "user_id" => user_id,
        },
        |(id, user_id, goods_id, deal_price, deal_time, status)| {
            Orders {
                id: Some(id),
                user_id,
                goods_id,
                deal_price,
                deal_time,
                status,
            }
        },
    )?;
    Ok(result)
}

// 查询指定查询条件的商品
// 查询条件: type, brand, max_price, min_price , fineness, desc, location
// 当查询条件为None时, 表示不限制查询条件
// 当max_price和min_price都为None时, 表示不限制价格
// 当max_price为None, min_price不为None时, 表示价格大于等于min_price
// 当max_price不为None, min_price为None时, 表示价格小于等于max_price
// 当max_price和min_price都不为None时, 表示价格在min_price和max_price之间

pub fn query_goods_by_condition(type_: Option<&str>, brand: Option<&str>, max_price: Option<f32>, min_price: Option<f32>, fineness: Option<&str>, desc: Option<&str>, location: Option<&str>) -> Result<Vec<Goods>, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_map(
        r"SELECT * FROM goods WHERE type = IFNULL(:type, type) AND brand = IFNULL(:brand, brand) AND price <= IFNULL(:max_price, price) AND price >= IFNULL(:min_price, price) AND fineness = IFNULL(:fineness, fineness) AND desc = IFNULL(:desc, desc) AND location = IFNULL(:location, location)",
        params! {
            "type" => type_,
            "brand" => brand,
            "max_price" => max_price,
            "min_price" => min_price,
            "fineness" => fineness,
            "desc" => desc,
            "location" => location,
        },
        |(id, type_, brand, price, fineness, desc, location)| {
            Goods {
                id: Some(id),
                type_,
                brand,
                price,
                fineness,
                desc,
                location,
            }
        },
    )?;
    Ok(result)
}


// 查询指定查询条件的订单
// 查询条件: user_id, goods_id, max_price, min_price, deal_time_begin, deal_time_end , status
// 当查询条件为None时, 表示不限制查询条件
// 当max_price和min_price同上
// 当deal_time_begin和deal_time_end都为None时, 表示不限制交易时间
// 其余同上

pub fn query_orders_by_condition(user_id: Option<i32>, goods_id: Option<i32>, max_price: Option<f32>, min_price: Option<f32>, deal_time_begin: Option<&str>, deal_time_end: Option<&str>, status: Option<&str>) -> Result<Vec<Orders>, Error> {
    let pool = pool();
    let mut conn = pool.get_conn()?;
    let result = conn.exec_map(
        r"SELECT * FROM orders WHERE user_id = IFNULL(:user_id, user_id) AND goods_id = IFNULL(:goods_id, goods_id) AND deal_price <= IFNULL(:max_price, deal_price) AND deal_price >= IFNULL(:min_price, deal_price) AND deal_time >= IFNULL(:deal_time_begin, deal_time) AND deal_time <= IFNULL(:deal_time_end, deal_time) AND status = IFNULL(:status, status)",
        params! {
            "user_id" => user_id,
            "goods_id" => goods_id,
            "max_price" => max_price,
            "min_price" => min_price,
            "deal_time_begin" => deal_time_begin,
            "deal_time_end" => deal_time_end,
            "status" => status,
        },
        |(id, user_id, goods_id, deal_price, deal_time, status)| {
            Orders {
                id: Some(id),
                user_id,
                goods_id,
                deal_price,
                deal_time,
                status,
            }
        },
    )?;
    Ok(result)
}





