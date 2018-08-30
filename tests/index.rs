#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate json_gettext;
extern crate rocket;

extern crate rocket_json_response;

use rocket::local::Client;
use rocket::http::Status;

use rocket_json_response::*;
use json_gettext::Value;

#[get("/status")]
fn status() -> JSONResponseWithoutData {
    JSONResponseWithoutData::ok()
}

#[test]
fn test_status_ok() {
    let rocket = rocket::ignite();

    let rocket = rocket
        .mount("/", routes![status]);

    let client = Client::new(rocket).expect("valid rocket instance");

    let req = client.get("/status");

    let mut response = req.dispatch();

    assert_eq!(response.body_string(), Some("{\"code\":0}".to_string()));
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap().to_string(), "application/json");
}

struct AccountNotFoundError;

impl JSONResponseCode for AccountNotFoundError {
    fn get_code(&self) -> i32 {
        100
    }
}

#[get("/error-code")]
fn error_code() -> JSONResponseWithoutData {
    JSONResponseWithoutData::err(AccountNotFoundError)
}

#[test]
fn test_error_code() {
    let rocket = rocket::ignite();

    let rocket = rocket
        .mount("/", routes![error_code]);

    let client = Client::new(rocket).expect("valid rocket instance");

    let req = client.get("/error-code");

    let mut response = req.dispatch();

    assert_eq!(response.body_string(), Some("{\"code\":100}".to_string()));
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap().to_string(), "application/json");
}

#[get("/me/name")]
fn me_name<'a>() -> JSONResponse<'a> {
    JSONResponse::ok(Value::from_str("Magic Len"))
}

#[test]
fn test_me_name() {
    let rocket = rocket::ignite();

    let rocket = rocket
        .mount("/", routes![me_name]);

    let client = Client::new(rocket).expect("valid rocket instance");

    let req = client.get("/me/name");

    let mut response = req.dispatch();

    assert_eq!(response.body_string(), Some("{\"code\":0,\"data\":\"Magic Len\"}".to_string()));
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap().to_string(), "application/json");
}