# SQL初始化

确保mysql已安装  
Ubuntu安装命令：
```bash
    sudo apt-get update
    sudo apt-get install mysql-server
    sudo systemctl start mysql
    sudo systemctl enable mysql
```

设置密码
```bash
    sudo mysql -uroot -p
```
```SQL
    -- 新安装修改密码
    ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY 'passwd';
    -- 创建用户cat
    CREATE USER 'cat'@'localhost' IDENTIFIED BY 'cat0520';
    GRANT ALL PRIVILEGES ON *.* TO 'cat'@'localhost' WITH GRANT OPTION;
    FLUSH PRIVILEGES;
    EXIT
```


进入SQL文件夹,命令行执行
```bash
    mysql -u cat -p < ./create_db.sql
```
