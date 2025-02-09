use mysql_async::*;
use mysql_async::Pool;
use mysql_async::Error;
use mysql_async::prelude::*;
use crate::models::User;
use crate::models::Good;
use crate::models::Orders;
use crate::models::Store;
use std::result::Result;

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

pub async fn create_user_by_open_id(pool: &Pool,open_id: &str)-> Result<(), Error> {

    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"INSERT INTO users (open_id) VALUES (:open_id)",
        params! {
            "open_id" => open_id,
        },
    ).await?;
    Ok(())
}


pub async fn drop_user_by_open_id(pool: &Pool,open_id: &str) -> Result<(), Error> {

    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"DELETE FROM users WHERE open_id = :open_id",
        params! {
            "open_id" => open_id,
        },
    ).await?;
    Ok(())
}


pub async fn update_user_by_open_id(pool: &Pool,open_id: &str, name: &str, phone: &str, stores_id: &str) -> Result<(), Error> {
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"UPDATE users SET name = :name, phone = :phone, stores_id = :stores_id WHERE open_id = :open_id",
        params! {
            "name" => name,
            "phone" => phone,
            "stores_id" => stores_id,
            "open_id" => open_id,
            },
    ).await?;
    Ok(())
}


pub async fn get_user_by_open_id(pool: &Pool, open_id: &str) -> Result<Option<User>, Error> {
    let mut conn = pool.get_conn().await?;

    let user: Option<User> = conn.exec_first(
        "SELECT id, open_id, username FROM users WHERE open_id = :open_id",
        params! { "open_id" => open_id },
    )
    .await?;

    Ok(user)
}

pub async fn set_user_name(pool: &Pool,open_id: &str,name: &str)-> Result<(),Error>{
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"UPDATE users SET name = :name WHERE open_id = :open_id",
        params! {
            "name" => name,
            "open_id" => open_id,
            },
    ).await?;
    Ok(())
}
pub async fn set_user_phone(pool: &Pool,open_id: &str,phone: &str)-> Result<(),Error>{
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"UPDATE users SET phone = :phone WHERE open_id = :open_id",
        params! {
            "phone" => phone,
            "open_id" => open_id,
            },
    ).await?;
    Ok(())
}
pub async fn set_user_stores_id(pool: &Pool,open_id: &str,stores_id: &str)-> Result<(),Error>{
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        r"UPDATE users SET stores_id = :stores_id WHERE open_id = :open_id",
        params! {
            "stores_id" => stores_id,
            "open_id" => open_id,
            },
    ).await?;
    Ok(())
}


