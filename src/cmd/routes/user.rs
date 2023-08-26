use rocket::{get, Route, routes};
use rocket_contrib::json::Json;
use rocket::response::content;
use rocket::response::content::RawJson;
use crate::common::pagination::{PageDto, PageOptions};
use crate::common::response::GenericResponse;
use crate::models::user::User;
use crate::services::handlers::user::user_handler::get_all_user_srv;


#[get("/get-all")]
pub fn get_all_user (options: PageOptions) -> RawJson<GenericResponse<User>> {
    let query_result = get_all_user_srv(options);
    if query_result.data.len() > 0 {
        RawJson(GenericResponse::new(
            String::from("200"),
            String::from("Success"),
            String::from(""),
            query_result,
        ))
    } else {
        RawJson(GenericResponse::new(
            String::from("404"),
            String::from("Not Found"),
            String::from(""),
            query_result,
        ))
    }
}
#[get("/get-by-id")]
pub fn get_by_id () {
    // let query_result = get_all_user_srv();
    // if(query_result.len() > 0) {
    //     Json(query_result)
    // } else {
    //     Json(vec![])
    // }
}

pub fn routes() -> Vec<Route> {
    routes![get_all_user, get_by_id]
}

// #[get("/get_by_id/<id>")]
// pub async fn get_by_id_user (id: i32) -> Result<Json<&'static str>, _> {
//     Ok(Json("response"))
// }
//
// #[post("/create")]
// pub async fn create_user () -> Result<Json<&'static str>, _> {
//     Ok(Json("response"))
// }
// #[patch("/update/<id>")]
// pub async fn update_user (id: i32) -> Result<Json<&'static str>, _> {
//     Ok(Json("response"))
// }
// #[delete("/delete/<id>")]
// pub async fn delete_user (id: i32) -> Result<Json<&'static str>, _> {
//     Ok(Json("response"))
// }