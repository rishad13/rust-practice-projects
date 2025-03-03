use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: serde_json::Value,
    pub message: String,
    pub status: bool,
    response_code: StatusCode,
}
#[derive(Serialize, Deserialize)]
struct ResponsesModel {
    status: bool,
    message: String,
    body: serde_json::Value,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: serde_json::Value, message: String, status: bool) -> Self {
        ApiResponse {
            status_code,
            body,
            message,
            status,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let _ = req;
        let responses = ResponsesModel {
            body: self.body,
            message: self.message,
            status: self.status,
        };
        let res_str = serde_json::to_string(&responses);
        let body = BoxBody::new(web::BytesMut::from(res_str.unwrap().as_bytes()));

        HttpResponse::new(self.response_code).set_body(body)
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {} \n Status Code: {}",
            self.body, self.status_code
        )
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_str().unwrap().as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}
