#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate rocket_json_response;

use rocket_json_response::{JSONResponse, JSONResponseWithoutData};

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

fn main() {
    rocket::ignite()
        .mount("/", routes![alive])
        .mount("/", routes![id])
        .mount("/", routes![id_str])
        .launch();
}