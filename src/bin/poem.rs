use std::net::IpAddr;
use poem::listener::TcpListener;
use poem::Route;
use poem::web::Query;
use poem_openapi::{OpenApi, OpenApiService};
use poem_openapi::payload::PlainText;
use poem_openapi::types::ToJSON;

struct Api;
// mod user;

#[OpenApi]
impl Api{
    #[oai(path="/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>)->PlainText<String>{
        println!("name={}",name.to_json_string());
        match name.0 {
            Some(name) =>PlainText(format!("hello, {}",name)),
            None=>PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main()->Result<(), std::io::Error>{
    const  PORT: i32 = 3000;
    const IP: &str ="127.0.0.1";
    let server_url: String = format!("{IP}:{PORT}");


    let api_service = OpenApiService::new(Api, "hello world", "1.0").server(format!("http://localhost:{PORT}/api"));
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/",ui);

    println!("Server started on {server_url}");
    poem::Server::new(TcpListener::bind(format!("127.0.0.1:{PORT}"))).run(app).await
}