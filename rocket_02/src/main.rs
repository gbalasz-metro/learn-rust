#![allow(unused)]

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket;

const RELEASES_PREFIX: Origin<'static> = uri!("/releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(RELEASES_PREFIX, keep("fedora","37", msg)))
}

#[get("/keep/<_platform>/<version>?<_msg>")]
fn keep(_platform: &str, version: &str, _msg: Option<&str>) -> Result<Value, Status> {
    if version.eq("35") {
        println!("msg: {version}");
        return Err(Status::NoContent);
    }
    Ok(json!({
        "notes": _msg,
        "platform": _platform,
        "version": version
    }))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(RELEASES_PREFIX, routes![keep])
}