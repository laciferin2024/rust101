use std::ffi::CString;
use poem_openapi::{Object, OpenApi};
use poem_openapi::payload::Json;

pub struct UserApi;

#[derive(Object)]
struct User{
    id:Option<i32>,
    name: String,
    color:String,
    haircolor:String
}

#[OpenApi]
impl UserApi{

    #[oai(path="/user", method="post")]
    async fn userdisplay(&self, mut user:Json<User>)-> Json<User>{
        user.0.id = Some(420);
        Json(user.0)
    }
}