use chrono::NaiveDateTime;
use diesel::{Associations, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::models::user::User;
#[derive(Queryable, Associations, Selectable, Debug, PartialEq, Clone,Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(User))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub created_by: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i32>,
}