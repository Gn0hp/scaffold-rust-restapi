use std::io::Cursor;
use std::time::SystemTime;
use rocket::{Request, Response};
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};
use crate::common::pagination::{PageDto, PageMetaData};

#[derive(Serialize, Deserialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: String,
    pub timestamp: String,
    pub response: PageDto<T>,
}

impl<T> GenericResponse<T> {
    pub fn new(status: String, message: String, timestamp: String, response: PageDto<T>) -> Self {
        GenericResponse {
            status,
            message,
            timestamp,
            response,
        }
    }
    pub fn default() -> Self {
        GenericResponse {
            status: String::from("200"),
            message: String::from("Success"),
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string(),
            response: PageDto::new(
                PageMetaData::default(),
                Vec::new(),
            ),
        }
    }
    pub fn default_error(message: String) -> Self {
        GenericResponse {
            status: String::from("404"),
            message,
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string(),
            response: PageDto::new(
                PageMetaData::default(),
                Vec::new(),
            )
        }
    }
}
impl<'r, T: Serialize> Responder<'r, 'static> for GenericResponse<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build().
            header(rocket::http::ContentType::JSON).
            status(rocket::http::Status::Ok).
            streamed_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}