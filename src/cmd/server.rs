use rocket::{Build, Rocket, routes};
use crate::cmd::routes::user::get_all_user;

pub fn server() -> Rocket<Build> {
    rocket::build()
        .mount("/users", routes![get_all_user])
        // .mount("/posts", routes![])
}