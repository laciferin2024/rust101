use std::ffi::CString;

pub struct UserApi;

#[derive(Object)]
struct User{
    id:Option<i32>
    name: String,
    color:CString,
    haircolor:String
}

#[OpenApi]
impl UserApi{

    #[oai(path="/user", method="post")]
    async fn userdisplay(&self, mut user:Json<User>)-> Json<user>{
        user.0.id = Some(420);
        Json(user.0)
    }
}