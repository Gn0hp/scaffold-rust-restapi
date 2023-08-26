use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub gender: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub created_by: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i32>,
}