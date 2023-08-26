use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct JwtPayload {
    pub id: i32,
    pub username: String,
    pub email: String,
}
pub struct Token(String);

#[derive(Debug)]
pub enum AccessTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = AccessTokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                let token = token.replace("Bearer ", "");
                //check here
                Outcome::Success(Token(token))
            },
            None => Outcome::Failure((rocket::http::Status::Unauthorized, AccessTokenError::Missing)),
        }
    }
}