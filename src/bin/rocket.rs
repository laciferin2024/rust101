#[macro_use]
extern crate rocket;

use std::fmt::format;
use rocket::http::Status;
use rocket::http::uri::Origin;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

const URI_RELEASES_PREFIX: Origin<'static> = uri!("/releases");

#[get("/")]
fn index() -> Redirect {
    let msg : Option<&str> = None;
    Redirect::to(uri!(URI_RELEASES_PREFIX, releases("osx", "v1.0",msg)))
}

#[get("/")]
fn hello() -> String {
    String::from("Hello")
}


#[get("/<platform>/<version>?<msg>")]
fn releases(platform:&str, version:&str,msg: Option<String>)->Result<Value,Status>{

    if let Some(msg) = msg {
        println!("msg is {msg}");
        return Err(Status::NoContent);
    }

    Ok(json!({
        "notes": "ready",
        "platform": platform,
        "version": version,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().
        mount("/", routes![index]).
        mount("/hello", routes![hello]).
        mount(URI_RELEASES_PREFIX.to_string(), routes![releases])
        // mount("/releases", routes![releases])
}
