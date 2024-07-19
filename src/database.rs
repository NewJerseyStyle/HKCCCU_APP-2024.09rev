#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

// 建立连接
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    // 从数据库中拿到环境变量
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // 建连MySQL连接
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
