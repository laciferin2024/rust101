use poem::listener::TcpListener;
use poem::Route;
use poem::web::Query;
use poem_openapi::{OpenApi, OpenApiService};
use poem_openapi::payload::PlainText;

struct Api;
// mod user;

#[OpenApi]
impl Api{
    #[oai(path="/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>)->PlainText<String>{
        match name.0 {
            Some(name) =>PlainText(format!("hello, {}",name)),
            None=>PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main()->Result<(), std::io::Error>{
    let api_service = OpenApiService::new(Api, "hello world", "1.0").server("http://localhost:3001/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/",ui);
    poem::Server::new(TcpListener::bind("127.0.0.1:3001")).run(app).await
}