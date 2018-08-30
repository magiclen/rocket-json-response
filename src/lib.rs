//! # JSON Response for Rocket Framework
//! This is a crate which provides `JSONResponse` and `JSONResponseWithoutData` structs to response JSON format data with an additional **code** integer value.
//!
//! Typically, the code **0** means **OK**. You can define other codes by yourself by implementing `JSONResponseCode` trait for your struct.

extern crate json_gettext;
extern crate rocket;

use std::io::Cursor;

pub use json_gettext::Value;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};

#[doc(hidden)]
pub const JSON_RESPONSE_CHUNK_SIZE: u64 = 4096;

/// The code of your JSON response.
pub trait JSONResponseCode {
    /// Assume the code **0** means **OK**. You can define other codes by yourself.
    /// This method will be called for one time when the response is being triggered. You can do something (perhaps keep a log?) at the moment.
    fn get_code(&self) -> i32;
}

struct CodeOK;

impl JSONResponseCode for CodeOK {
    fn get_code(&self) -> i32 {
        0
    }
}

/// To respond JSON data.
///
/// The format of JSON data looks like,
///
/// ```json
/// {
///    code: <integer>,
///    data: <json value>
/// }
/// ```
pub struct JSONResponse<'a> {
    pub code: Box<JSONResponseCode>,
    pub data: Value<'a>,
}

impl<'a> Responder<'a> for JSONResponse<'a> {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let json = format!("{{\"code\":{},\"data\":{}}}", self.code.get_code(), self.data.to_json());

        let mut response = Response::build();

        response
            .raw_header("Content-Type", "application/json")
            .raw_header("Content-Length", json.len().to_string())
            .chunked_body(Cursor::new(json), JSON_RESPONSE_CHUNK_SIZE);

        response.ok()
    }
}

impl<'a> JSONResponse<'a> {
    pub fn ok(data: Value<'a>) -> Self {
        JSONResponse {
            code: Box::new(CodeOK),
            data,
        }
    }

    pub fn err<T: JSONResponseCode + 'static>(code: T, data: Value<'a>) -> Self {
        JSONResponse {
            code: Box::new(code),
            data,
        }
    }
}

/// To respond JSON data.
///
/// The format of JSON data looks like,
///
/// ```json
/// {
///    code: <integer>
/// }
/// ```
pub struct JSONResponseWithoutData {
    pub code: Box<JSONResponseCode>
}

impl JSONResponseWithoutData {
    pub fn ok() -> Self {
        JSONResponseWithoutData { code: Box::new(CodeOK) }
    }

    pub fn err<T: JSONResponseCode + 'static>(code: T) -> Self {
        JSONResponseWithoutData { code: Box::new(code) }
    }
}

impl<'a> Responder<'a> for JSONResponseWithoutData {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let json = format!("{{\"code\":{}}}", self.code.get_code());

        let mut response = Response::build();

        response
            .raw_header("Content-Type", "application/json")
            .raw_header("Content-Length", json.len().to_string())
            .chunked_body(Cursor::new(json), JSON_RESPONSE_CHUNK_SIZE);

        response.ok()
    }
}