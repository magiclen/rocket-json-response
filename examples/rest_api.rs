#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_json_response;

use rocket_json_response::{JSONResponse, JSONResponseWithoutData};

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

serialize_to_json!(User);

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
    JSONResponse::err(1, id) // The code "1" means the id is not a u32 value
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