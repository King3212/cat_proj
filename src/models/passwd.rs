/* 
#   password Table
#   
#   +----+--------------+------+------------+
#   | id | user_id(uni) | salt | local_hash |
#   +----+--------------+------+------------+
#
*/

use mysql_async::prelude::FromRow;
use mysql_async::Row;

// users Table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Passwd {
    pub id: i32,
    pub user_id: String,
    pub salt: String,
    pub local_hash: String,
}


impl FromRow for Passwd {
    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (id, user_id, salt, local_hash) = mysql_async::from_row(row);
        Ok(Passwd {
            id,
            user_id,
            salt,
            local_hash,
        })
    }

    fn from_row(row: Row) -> Self
        where
            Self: Sized, {
        let (id, user_id, salt, local_hash) = mysql_async::from_row(row);
        Passwd {
            id,
            user_id,
            salt,
            local_hash,
        }
    }
}


use mysql_async::Pool;
use mysql_async::Error;
use mysql_async::prelude::*;
use std::result::Result;

pub async fn get_passwd_by_user_id(pool:&Pool, user_id: &str) -> Result<Option<Passwd>, Error> {
    let mut conn = pool.get_conn().await?;
    let query = "SELECT * FROM passwd WHERE user_id = ?";
    let passwd = conn.exec_first(query, (user_id,)).await?;
    Ok(passwd)
}