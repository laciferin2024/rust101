#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::State;

const URI_RELEASES_PREFIX: Origin<'static> = uri!("/releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(URI_RELEASES_PREFIX, releases("osx", "v1.0", msg)))
}

#[get("/")]
fn hello() -> String {
    String::from("Hello")
}

async fn get_latest_release(client: &State<Client>, repo: &str) -> Result<Value, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let github_release = response.json::<Value>().await?;
    Ok(github_release)
}

const REPO_GOLANG_AIR: &str = "air-verse/air";

#[get("/<platform>/<version>?<msg>")]
async fn releases(
    platform: &str,
    version: &str,
    msg: Option<String>,
    client: &State<Client>,
) -> Result<Value, Status> {
    if let Some(msg) = msg {
        println!("msg is {msg}");
        return Err(Status::NoContent);
    }

    // let response = get_latest_release(client,REPO_GOLANG).await.or(Err(Status::NoContent));
    let response = get_latest_release(client, REPO_GOLANG_AIR).await;

    match response {
        Err(e) => {
            println!("request errored with {}", e);
            return Err(Status::NoContent);
        }
        Ok(response) => Ok(json!({
            "notes": "ready",
            "platform": platform,
            "version": version,
            "response": response ,
        })),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Client::builder().user_agent("reqwest").build().unwrap())
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .mount(URI_RELEASES_PREFIX.to_string(), routes![releases])
    // mount("/releases", routes![releases])
}
