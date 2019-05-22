/*!
# JSON Response for Rocket Framework

This is a crate which provides `JSONResponse` and `JSONResponseWithoutData` structs to response JSON format data with an additional **code** integer value.

Typically, the code **0** means **OK**. You can define other codes by yourself by implementing `JSONResponseCode` trait for your struct.

See `examples`.
*/

pub extern crate json_gettext;
extern crate rocket;

mod json_response_code;
mod to_json;

use std::marker::PhantomData;

pub use json_response_code::JSONResponseCode;
pub use to_json::ToJSON;

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
pub struct JSONResponse<'a, T: ToJSON = JSONGetTextValue<'a>> {
    pub code: Box<JSONResponseCode>,
    pub data: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: ToJSON> Responder<'a> for JSONResponse<'a, T> {
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

impl<'a, T: ToJSON> JSONResponse<'a, T> {
    pub fn ok(data: T) -> Self {
        JSONResponse {
            code: Box::new(0),
            data,
            phantom: PhantomData,
        }
    }

    pub fn err<K: JSONResponseCode + 'static>(code: K, data: T) -> Self {
        JSONResponse {
            code: Box::new(code),
            data,
            phantom: PhantomData,
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

    pub fn err<K: JSONResponseCode + 'static>(code: K) -> Self {
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