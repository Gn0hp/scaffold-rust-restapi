use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;
use diesel::Connection;

pub fn establish_connection() -> MysqlConnection{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}