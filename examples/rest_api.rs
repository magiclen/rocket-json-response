#[macro_use]
extern crate rocket;

#[macro_use]
extern crate enum_ordinalize;

use rocket_json_response::{
    serialize_to_json, JSONResponse, JSONResponseCode, JSONResponseWithoutData,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id:   i32,
    name: String,
}

serialize_to_json!(User);

#[derive(Ordinalize)]
#[repr(i32)]
pub enum ErrorCode {
    IncorrectIDFormat = 100,
}

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
        id: 0, name: "Magic Len".to_string()
    })
}

use rocket::{
    http::Status,
    outcome::Outcome,
    request::{FromRequest, Outcome as RequestOutcome, Request},
};

struct UserAgent<'a> {
    user_agent: &'a str,
}

#[async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> RequestOutcome<Self, Self::Error> {
        let user_agent: Option<&str> = request.headers().get("user-agent").next();

        match user_agent {
            Some(user_agent) => Outcome::Success(UserAgent {
                user_agent,
            }),
            None => Outcome::Forward(Status::BadRequest),
        }
    }
}

#[get("/client/user-agent")]
fn user_agent(user_agent: UserAgent) -> JSONResponse<&str> {
    JSONResponse::ok(user_agent.user_agent)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![alive])
        .mount("/", routes![id, id_str])
        .mount("/", routes![user])
        .mount("/", routes![user_agent])
}
