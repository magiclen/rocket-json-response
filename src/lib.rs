/*!
# JSON Response for Rocket Framework

This is a crate which provides `JSONResponse` and `JSONResponseWithoutData` structs to response JSON format data with an additional **code** integer value.

Typically, the code **0** means **OK**. You can define other codes by yourself by implementing `JSONResponseCode` trait for your struct.

See `examples`.
*/

pub extern crate json_gettext;

mod json_response_code;
mod to_json;

use std::{
    fmt::{self, Debug, Formatter},
    io::Cursor,
    marker::PhantomData,
};

use json_gettext::JSONGetTextValue;
pub use json_response_code::JSONResponseCode;
use rocket::{
    request::Request,
    response::{self, Responder, Response},
};
pub use to_json::ToJSON;

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
    code:    Box<dyn JSONResponseCode>,
    data:    T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: ToJSON> Debug for JSONResponse<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_helper::impl_debug_for_struct!(JSONResponse, f, self, let .code = self.code.get_code(), let .data = self.data.to_json());
    }
}

impl<'a, T: ToJSON> JSONResponse<'a, T> {
    #[inline]
    pub fn ok(data: T) -> Self {
        JSONResponse {
            code: Box::new(0),
            data,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn err<K: JSONResponseCode + 'static>(code: K, data: T) -> Self {
        JSONResponse {
            code: Box::new(code),
            data,
            phantom: PhantomData,
        }
    }
}

impl<'r, 'o: 'r, T: ToJSON> Responder<'r, 'o> for JSONResponse<'o, T> {
    fn respond_to(self, _: &Request) -> response::Result<'o> {
        let json =
            format!("{{\"code\":{},\"data\":{}}}", self.code.get_code(), self.data.to_json());

        let mut response = Response::build();

        response
            .raw_header("Content-Type", "application/json")
            .sized_body(json.len(), Cursor::new(json));

        response.ok()
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
    code: Box<dyn JSONResponseCode>,
}

impl Debug for JSONResponseWithoutData {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_helper::impl_debug_for_struct!(JSONResponseWithoutData, f, self, let .code = self.code.get_code());
    }
}

impl JSONResponseWithoutData {
    #[inline]
    pub fn ok() -> Self {
        JSONResponseWithoutData {
            code: Box::new(0)
        }
    }

    #[inline]
    pub fn err<K: JSONResponseCode + 'static>(code: K) -> Self {
        JSONResponseWithoutData {
            code: Box::new(code)
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for JSONResponseWithoutData {
    fn respond_to(self, _: &Request) -> response::Result<'o> {
        let json = format!("{{\"code\":{}}}", self.code.get_code());

        let mut response = Response::build();

        response
            .raw_header("Content-Type", "application/json")
            .sized_body(json.len(), Cursor::new(json));

        response.ok()
    }
}
