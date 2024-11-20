use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::json;

#[derive(Debug)]
pub struct ApiError {
    code: Status,
    message: Option<String>,
}

impl ApiError {
    pub fn custom(code: Status, message: Option<String>) -> Self {
        ApiError { code, message }
    }

    pub fn unauthorized(message: Option<String>) -> Self {
        ApiError {
            code: Status::Unauthorized,
            message,
        }
    }

    pub fn not_found(message: Option<String>) -> Self {
        ApiError {
            code: Status::NotFound,
            message,
        }
    }

    pub fn internal() -> Self {
        ApiError {
            code: Status::InternalServerError,
            message: None,
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body: String = match self.message {
            Some(message) => json!({ "code": self.code.code, "message": message }),
            None => json!({ "code": self.code.code }),
        }
        .to_string();

        Response::build()
            .header(ContentType::JSON)
            .status(self.code)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
