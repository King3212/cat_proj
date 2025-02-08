use mysql_async::*;
use mysql_async::Pool;
use mysql_async::Error;
use mysql_async::prelude::*;
use crate::models::Users;
use crate::models::Goods;
use crate::models::Orders;
use crate::DATABASE_URL;
use std::result::Result;

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

pub async fn create_user(name: &str, password: &str, phone: &str, wx_id: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"INSERT INTO users (name, password, phone, wx_id) VALUES (:name, :password, :phone, :wx_id)",
        params! {
            "name" => name,
            "password" => password,
            "phone" => phone,
            "wx_id" => wx_id,
        },
    ).await?;
    Ok(())
}

// 创建商品

pub async fn create_goods(type_: &str, brand: &str, price: f32, fineness: &str, desc: &str, location: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"INSERT INTO goods (type, brand, price, fineness, desc, location) VALUES (:type, :brand, :price, :fineness, :desc, :location)",
        params! {
            "type" => type_,
            "brand" => brand,
            "price" => price,
            "fineness" => fineness,
            "desc" => desc,
            "location" => location,
        },
    ).await?;
    Ok(())
}

// 创建订单

pub async fn create_orders(user_id: i32, goods_id: i32, deal_price: f32, deal_time: &str, status: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"INSERT INTO orders (user_id, goods_id, deal_price, deal_time, status) VALUES (:user_id, :goods_id, :deal_price, :deal_time, :status)",
        params! {
            "user_id" => user_id,
            "goods_id" => goods_id,
            "deal_price" => deal_price,
            "deal_time" => deal_time,
            "status" => status,
        },
    ).await?;
    Ok(())
}

// 查询用户

pub async fn query_user(wx_id: &str) -> Result<Users, Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    let row = conn.exec_first(
        r"SELECT * FROM users WHERE wx_id = :wx_id",
        params! {
            "wx_id" => wx_id,
        },
    ).await?;
    let row:Row = row.unwrap();
    let user = Users {
        id: row.get("id").unwrap(),
        name: row.get("name").unwrap(),
        password: row.get("password").unwrap(),
        phone: row.get("phone").unwrap(),
        wx_id: row.get("wx_id").unwrap(),
    };
    Ok(user)
}

// 查询商品

pub async fn query_goods(id: i32) -> Result<Goods, Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    let row = conn.exec_first(
        r"SELECT * FROM goods WHERE id = :id",
        params! {
            "id" => id,
        },
    ).await?;
    let row:Row = row.unwrap();
    let goods = Goods {
        id: row.get("id").unwrap(),
        type_: row.get("type").unwrap(),
        brand: row.get("brand").unwrap(),
        price: row.get("price").unwrap(),
        fineness: row.get("fineness").unwrap(),
        desc: row.get("desc").unwrap(),
        location: row.get("location").unwrap(),
    };
    Ok(goods)
}

// 查询订单

pub async fn query_orders(id: i32) -> Result<Orders, Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    let row = conn.exec_first(
        r"SELECT * FROM orders WHERE id = :id",
        params! {
            "id" => id,
        },
    ).await?;
    let row:Row = row.unwrap();
    let orders = Orders {
        id: row.get("id").unwrap(),
        user_id: row.get("user_id").unwrap(),
        goods_id: row.get("goods_id").unwrap(),
        deal_price: row.get("deal_price").unwrap(),
        deal_time: row.get("deal_time").unwrap(),
        status: row.get("status").unwrap(),
    };
    Ok(orders)
}

// 更新用户

pub async fn update_user(name: &str, password: &str, phone: &str, wx_id: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"UPDATE users SET password = :password, phone = :phone, wx_id = :wx_id WHERE name = :name",
        params! {
            "name" => name,
            "password" => password,
            "phone" => phone,
            "wx_id" => wx_id,
        },
    ).await?;
    Ok(())
}

// 更新商品

pub async fn update_goods(id: i32, type_: &str, brand: &str, price: f32, fineness: &str, desc: &str, location: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
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
    ).await?;
    Ok(())
}

// 更新订单

pub async fn update_orders(id: i32, user_id: i32, goods_id: i32, deal_price: f32, deal_time: &str, status: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"UPDATE orders SET user_id = :user_id, goods_id = :goods_id, deal_price = :deal_price, deal_time = :deal_time, status = :status WHERE id = :id",
        params! {
            "id" => id,
            "user_id" => user_id,
            "goods_id" => goods_id,
            "deal_price" => deal_price,
            "deal_time" => deal_time,
            "status" => status,
        },
    ).await?;
    Ok(())
}

// 删除用户

pub async fn delete_user(name: &str) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"DELETE FROM users WHERE name = :name",
        params! {
            "name" => name,
        },
    ).await?;
    Ok(())
}

// 删除商品

pub async fn delete_goods(id: i32) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"DELETE FROM goods WHERE id = :id",
        params! {
            "id" => id,
        },
    ).await?;
    Ok(())
}

// 删除订单

pub async fn delete_orders(id: i32) -> Result<(), Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"DELETE FROM orders WHERE id = :id",
        params! {
            "id" => id,
        },
    ).await?;
    Ok(())
}

// 查询所有用户
pub async fn query_all_users() -> Result<Vec<Users>, mysql_async::Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    let mut rows = conn.exec_iter(
        r"SELECT * FROM users",
        (),
    ).await?;

    let mut users = Vec::new();

    rows.for_each(|row| {
        let user = Users {
            id: row.get("id").unwrap(),
            name: row.get("name").unwrap(),
            password: row.get("password").unwrap(),
            phone: row.get("phone").unwrap(),
            wx_id: row.get("wx_id").unwrap(),
        };
        users.push(user);
    }).await?;
    Ok(users)
}

// 通过筛选条件查询货物

// 筛选条件: type, brand, max_price, min_price, fineness
// 1. type, brand, fineness, max_price, min_price都是可选的
// 2. 如果type, brand, fineness, max_price, min_price都为空, 则返回所有货物
// 3. 最高价格和最低价格都是闭区间
// 4. 最高价格为0, 则不限制最高价格
// 5. 最低价格为0, 则不限制最低价格

pub async fn query_goods_by_filter(type_: &str, brand: &str, max_price: f32, min_price: f32, fineness: &str) -> Result<Vec<Goods>, mysql_async::Error> {
    let pool = Pool::new(DATABASE_URL.clone());
    let mut conn = pool.get_conn().await?;
    let mut sql = "SELECT * FROM goods WHERE 1 = 1".to_string();
    if type_ != "" {
        sql = format!("{} AND type = '{}'", sql, type_);
    }
    if brand != "" {
        sql = format!("{} AND brand = '{}'", sql, brand);
    }
    if fineness != "" {
        sql = format!("{} AND fineness = '{}'", sql, fineness);
    }
    if max_price != 0.0 {
        sql = format!("{} AND price <= {}", sql, max_price);
    }
    if min_price != 0.0 {
        sql = format!("{} AND price >= {}", sql, min_price);
    }
    let mut rows = conn.exec_iter(
        sql.as_str(),
        (),
    ).await?;

    let mut goods = Vec::new();

    rows.for_each(|row| {
        let good = Goods {
            id: row.get("id").unwrap(),
            type_: row.get("type").unwrap(),
            brand: row.get("brand").unwrap(),
            price: row.get("price").unwrap(),
            fineness: row.get("fineness").unwrap(),
            desc: row.get("desc").unwrap(),
            location: row.get("location").unwrap(),
        };
        goods.push(good);
    }).await?;
    Ok(goods)
}
