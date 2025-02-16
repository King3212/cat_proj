use mysql_async::prelude::FromRow;
use mysql_async::Row;

/*
#   stores Table
#   +----+------+---------+
#   | id | name | manager | 
#   +----+------+---------+
#
*/


//  stores Table
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
