/*!
# JSON Response for Rocket Framework

This is a crate which provides `JSONResponse` and `JSONResponseWithoutData` structs to response JSON format data with an additional **code** integer value.

Typically, the code **0** means **OK**. You can define other codes by yourself by implementing `JSONResponseCode` trait for your struct.

See `examples`.
*/

pub extern crate json_gettext;
extern crate rocket;

mod json_response_code;

pub use json_response_code::JSONResponseCode;

use std::io::Cursor;

use json_gettext::JSONGetTextValue;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};

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
    pub data: JSONGetTextValue<'a>,
}

impl<'a> Responder<'a> for JSONResponse<'a> {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let json = format!("{{\"code\":{},\"data\":{}}}", self.code.get_code(), self.data.to_json());

        let mut response = Response::build();

        response
            .raw_header("Content-Type", "application/json")
            .raw_header("Content-Length", json.len().to_string())
            .sized_body(Cursor::new(json));

        response.ok()
    }
}

impl<'a> JSONResponse<'a> {
    pub fn ok(data: JSONGetTextValue<'a>) -> Self {
        JSONResponse {
            code: Box::new(0),
            data,
        }
    }

    pub fn err<T: JSONResponseCode + 'static>(code: T, data: JSONGetTextValue<'a>) -> Self {
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
        JSONResponseWithoutData { code: Box::new(0) }
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
            .sized_body(Cursor::new(json));

        response.ok()
    }
}