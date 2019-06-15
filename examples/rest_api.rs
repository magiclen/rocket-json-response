#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_json_response;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate enum_ordinalize;

use rocket_json_response::{JSONResponse, JSONResponseWithoutData, JSONResponseCode};

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

serialize_to_json!(User);

create_ordinalized_enum!(ErrorCode: i32,
    IncorrectIDFormat = 100,
);

impl JSONResponseCode for ErrorCode {
    fn get_code(&self) -> i32 {
        self.ordinal()
    }
}

#[get("/")]
fn alive() -> JSONResponseWithoutData {
    JSONResponseWithoutData::ok()
}

#[get("/<id>")]
fn id(id: u32) -> JSONResponse<'static, u32> {
    JSONResponse::ok(id)
}

#[get("/<id>", rank = 1)]
fn id_str(id: String) -> JSONResponse<'static, String> {
    JSONResponse::err(ErrorCode::IncorrectIDFormat, id)
}

#[get("/user/magiclen")]
fn user() -> JSONResponse<'static, User> {
    JSONResponse::ok(User {
        id: 0,
        name: "Magic Len".to_string(),
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![alive])
        .mount("/", routes![id, id_str])
        .mount("/", routes![user])
        .launch();
}