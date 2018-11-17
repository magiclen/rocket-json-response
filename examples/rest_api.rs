#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rocket_json_response;

use rocket_json_response::{JSONResponse, JSONResponseWithoutData, json_gettext::JSONGetTextValue};

#[get("/")]
fn alive() -> JSONResponseWithoutData {
    JSONResponseWithoutData::ok()
}

#[get("/<id>")]
fn id(id: u32) -> JSONResponse<'static> {
    JSONResponse::ok(JSONGetTextValue::from_u32(id))
}

#[get("/<id>", rank = 1)]
fn id_str(id: String) -> JSONResponse<'static> {
    JSONResponse::err(1, JSONGetTextValue::from_string(id)) // The code "1" means the id is not a u32 value
}

fn main() {
    rocket::ignite()
        .mount("/", routes![alive])
        .mount("/", routes![id])
        .mount("/", routes![id_str])
        .launch();
}