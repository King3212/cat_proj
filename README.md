# cat_proj 项目


```
./src  
├── handlers                // 对接受到的请求做处理
│   ├── auth_handler.rs     // 处理认证
│   ├── good_handler.rs     // 处理货物请求
│   ├── mod.rs              // 模块文件
│   ├── store_handler.rs    // 处理商户请求
│   └── user_handler.rs     // 处理用户请求
├── main.rs
├── models
│   ├── good.rs
│   ├── jwt.rs
│   ├── mod.rs
│   ├── orders.rs
│   ├── store.rs
│   ├── user.rs
│   └── wx.rs
├── routes
│   ├── api_routes.rs
│   └── mod.rs
└── test                    
```