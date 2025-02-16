use mysql_async::prelude::FromRow;
use mysql_async::Row;
/*
#   orders Table
#
#   +----+----------+----------+-----------+-----------+
#   | id | users_id | goods_id | deal_time | stores_id | 
#   +----+----------+----------+-----------+-----------+
*/

//   orders Table
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

