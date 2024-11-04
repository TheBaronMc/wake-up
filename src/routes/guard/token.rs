use rocket::{outcome::Outcome, request::{self, FromRequest}, Request};
use rocket::http::Status;
use rocket::http::hyper::header;

use crate::{auth::verify_token, routes::errors::ApiError};

pub struct Token<'r>{
    _token: &'r str
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Token<'r>, ApiError> {
        match request.headers().get_one(header::AUTHORIZATION.as_str()) {
            Some(header) => {
                if header.starts_with("Bearer") {
                    let token = header[6..header.len()].trim();
                    if verify_token(token) {
                        Outcome::Success(Token{_token:token})
                    } else {
                        Outcome::Error((Status::Unauthorized, ApiError::unauthorized(Some(String::from("Unauthorized")))))
                    }
                } else {
                    Outcome::Error((Status::Unauthorized, ApiError::unauthorized(Some(String::from("Unauthorized")))))
                }
            },
            None => Outcome::Error((Status::Unauthorized, ApiError::unauthorized(Some(String::from("Unauthorized")))))
        }
    }
}