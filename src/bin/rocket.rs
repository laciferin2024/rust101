#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    String::from("Hello World")
}

#[get("/releases/<platform>/1.0?<msg>")]
fn releases(platform:&str, msg: Option<String>)->String{
    return String::from("Release")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
