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
DROP DATABASE IF EXISTS cat_proj;
CREATE DATABASE cat_proj;

USE cat_proj;

CREATE TABLE stores (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    manager_id INT UNIQUE  -- 先不添加外键
);

CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20) UNIQUE,  
    open_id VARCHAR(255) UNIQUE,  
    stores_id INT
);

-- 添加外键
ALTER TABLE users ADD CONSTRAINT fk_users_stores FOREIGN KEY (stores_id) REFERENCES stores(id) ON DELETE SET NULL;
ALTER TABLE stores ADD CONSTRAINT fk_stores_manager FOREIGN KEY (manager_id) REFERENCES users(id) ON DELETE SET NULL;


CREATE TABLE goods (
    id INT AUTO_INCREMENT PRIMARY KEY,
    type VARCHAR(100) NOT NULL,
    brand VARCHAR(100) NOT NULL,
    price_in DECIMAL(10,2) NOT NULL,
    price_out DECIMAL(10,2) NOT NULL,
    description TEXT,
    addr VARCHAR(255),
    pic_addr VARCHAR(255),
    fineness VARCHAR(50)
);

CREATE TABLE orders (
    id INT AUTO_INCREMENT PRIMARY KEY,
    users_id INT NOT NULL,
    goods_id INT NOT NULL,
    deal_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    stores_id INT NOT NULL,
    FOREIGN KEY (users_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (goods_id) REFERENCES goods(id) ON DELETE CASCADE,
    FOREIGN KEY (stores_id) REFERENCES stores(id) ON DELETE CASCADE
);

SHOW TABLES;
