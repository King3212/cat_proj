use crate::models::user;
use crate::POOL;

// // create user
// async fn create_user(info: web::Json<CreateAccountInfo>) -> impl Responder {
//     let user = info.into_inner();
//     let result = update_user_by_open_id(&POOL, &user.username, &user.password, &user.phone, &user.email).await;
//     match result {
//         Ok(_) => HttpResponse::Ok().body("User created successfully"),
//         Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
//     }
// }


// add user
pub async fn add_user_to_db_by_openid(open_id: &String){
    // 先查询有没有用户
    let user = user::get_user_by_open_id(&POOL,&open_id).await;
    match user {
        Ok(user) => {
            if user.is_none() {
                // 没有用户，添加用户
                user::create_user_by_open_id(&POOL,&open_id).await;
            }
        }
        Err(_) => {
            println!("查询用户失败");
        }
        
    }
}