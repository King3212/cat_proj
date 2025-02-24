use mysql_async::prelude::FromRow;
use mysql_async::Row;
/* 
#   users Table
#   
#   +----+------+------------+---------------+-----------+------+------------+
#   | id | name | phone(uni) | open_id(uni ) | stores_id | salt | local_hash |
#   +----+------+------------+---------------+-----------+------+------------+
#
*/

// users Table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub open_id: String,
    pub stores_id: i32,
    pub salt: String,
    pub local_hash: String,
}

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let (id, name, phone, open_id, stores_id, salt, local_hash) = mysql_async::from_row(row);
        User {
            id,
            name,
            phone,
            open_id,
            stores_id,
            salt,
            local_hash,
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
           let (id, name, phone, open_id, stores_id,salt,local_hash) = mysql_async::from_row_opt(row)?;
           Ok(User {
               id,
               name,
               phone,
               open_id,
               stores_id,
               salt,
               local_hash,
           })
    }
}


impl default::Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            phone: "".to_string(),
            open_id: "".to_string(),
            stores_id: 0,
            salt: "".to_string(),
            local_hash: "".to_string(),
        }
    }
}

use mysql_async::*;
use mysql_async::Pool;
use mysql_async::Error;
use mysql_async::prelude::*;

use std::default;
use std::result::Result;




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

pub async fn get_user_by_phone(pool: &Pool, phone: &str) -> Result<Option<User>, Error> {
    let mut conn = pool.get_conn().await?;
    let user: Option<User> = conn.exec_first(
        "SELECT id, open_id, username FROM users WHERE phone = :phone",
        params! { "phone" => phone },
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




