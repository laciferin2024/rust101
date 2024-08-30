#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::hash::Hash;
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
    // Ok(github_release)
    make_json_response(&github_release).ok_or(json!({})).or_else(|e| Ok(e))
}

const REPO_GOLANG_AIR: &str = "air-verse/air";


fn make_json_response(github_release: &Value) -> Option<Value> {
    let platforms_available: HashMap<&str, Vec<&str>> = HashMap::from([
        ("amd64.AppImage.tar.gz", vec!["linux-x86_64"]),
        ("app.tar.gz", vec!["darwin-x86_64", "darwin-aarch64"]),
        ("x64_en-US.msi.zip", vec!["windows-x86_64"])
    ]);

    let mut response = json!({
        "version": github_release["tag_name"].as_str()?,
        "notes": remove_suffix(github_release["body"].as_str()?,"See the assets to download this version and install.").trim_end_matches(['\r', '\n', ' ']), //TODO: suffix
        "pub_date": github_release["published_at"].as_str()?,
        "platforms": {},
    });

    let mut response_platforms = github_release["platforms"].as_object()?;


    for asset in github_release["assets"].as_array()?.iter() {
        let asset = asset.as_object()?;
        let browser_download_url = asset["browser_download_url"].as_str()?;
        let asset_name = asset["name"].as_str()?;
        for (extension, os_archs) in platforms_available.iter() {
            if asset_name.ends_with((extension)) {
                for os_arch in os_archs.iter()
                {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}))
                    }
                    response_platforms[os_arch.to_string()].as_object().insert("url".to_string(), browser_download_url);
                }
            } else if asset_name.ends_with(&format!("{extension}.sig")) {
                //     make a req to sig
                let sig = match text_request(client, browser_download_url) {
                    Ok(s) => s,
                    _ => String::new(),
                };
            }
        }
    }

    Some(response)
}

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


fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    s.strip_suffix(suffix).unwrap_or_else(|| s)
}

async fn text_request (client: &State<Client>, url : &str)->Result<String, reqwest::Error>{
    client.get(url).send().await?.text().await
}