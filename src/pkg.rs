
use std::fmt;
use std::error;

const OBS_API_URL: &str = "https://api.opensuse.org/";

pub fn get_pkg_version(pkgname: &str) -> String {
    if pkgname.is_empty() {
        return "No pkgname provided".to_string();
    }
    format!("Function under constructing!")
}

async fn query_pkg() -> Result<String, Box<dyn error::Error>>{
    let obs_username: String = match std::env::var("OBS_USERNAME") {
        Ok(t)  => t,
        Err(e) => panic!("OBS_USERNAME env variable not found. Error: {}", e),
    };
    let obs_password: String = match std::env::var("OBS_PASSWORD") {
        Ok(t)  => t,
        Err(e) => panic!("OBS_PASSWORD env variable not found. Error: {}", e),
    };
    println!("{:?}", obs_username);
    println!("{:?}", obs_password);
    let client = reqwest::Client::new();
    let url: String = OBS_API_URL.parse().unwrap();

    //client.get("https://httpbin.org/ip").basic_auth(obs_username, obs_password);
    Ok(reqwest::get("https://httpbin.org/ip").await?.text().await?)
}


#[tokio::test]
async fn query_test() {
    let result: String = match query_pkg().await{
        Ok(t) => t,
        Err(e) => panic!("Error: {}", e),
    };
    println!("{:?}", result);
}