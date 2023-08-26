use diesel::prelude::*;
use crate::common::pagination::{PageDto, PageMetaData, PageOptions};
use crate::models::user::User;
use crate::internals::db::mysql;
use crate::schema::users::dsl::*;

pub fn get_all_user_srv(options: PageOptions) -> PageDto<User> {
    let connection = &mut mysql::config::establish_connection();
    let query = users
        .order(created_at.desc())
        .limit(options.take as i64)
        .offset(options.skip() as i64);
    let res: Vec<User> = query.load::<User>(connection).expect("Error loading users");
    let page_meta_data = PageMetaData::new(options, res.len() as i32);
    PageDto::new(page_meta_data, res)
}
