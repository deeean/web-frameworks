#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response<'a> {
    message: &'a str,
}

#[get("/ping")]
fn pong() -> Json<Response<'static>> {
    Json(Response {
        message: "pong"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![pong])
}