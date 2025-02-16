use mysql_async::prelude::FromRow;
use mysql_async::Row;


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