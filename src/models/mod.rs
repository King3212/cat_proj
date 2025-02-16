pub mod user;
pub mod store;
pub mod good;
pub mod orders;
pub mod jwt;
pub mod wx;
pub mod passwd;


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