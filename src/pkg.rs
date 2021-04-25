use std::error;
use std::fmt;

//const OBS_API_URL: &str = "https://api.opensuse.org/";
const OBS_API_BASE: &str = r#"https://api.opensuse.org/search/published/binary/id?match=@name=""#;
const OBS_API_REST: &str = r#"" and (contains-ic(@arch, "x86_64") or contains-ic(@arch, "noarch")) and not(contains-ic(@project, "home:")) and not(contains-ic(@name, "-debuginfo")) and not(contains-ic(@name, "-debugsource")) and not(contains-ic(@name, "-devel")) and not(contains-ic(@name, "-lang")) and contains-ic(@baseproject, "openSUSE")"#;

pub fn get_pkg_version(pkgname: &str) -> String {
    if pkgname.is_empty() {
        "No pkgname provided.".to_string()
    } else {
        "Function under constructing!".to_string()
    }
}

async fn query_pkg(pkgname: &str) -> Result<String, reqwest::Error> {
    let obs_username: String = match std::env::var("OBS_USERNAME") {
        Ok(t)  => t,
        Err(e) => panic!("OBS_USERNAME env variable not found. Error: {}", e),
    };
    let obs_password: String = match std::env::var("OBS_PASSWORD") {
        Ok(t)  => t,
        Err(e) => panic!("OBS_PASSWORD env variable not found. Error: {}", e),
    };
    let client = reqwest::Client::new();
    let url = format!("{}{}{}", OBS_API_BASE, pkgname, OBS_API_REST);
    Ok(client.get(url).basic_auth(obs_username, Some(obs_password)).send().await?.text().await?)
}


#[tokio::test]
async fn query_test() {
    let result: String = match query_pkg("neofetch").await{
        Ok(t) => t,
        Err(e) => panic!("Error: {}", e),
    };
    println!("{:?}", &result);
    let result: String = match query_pkg("podman").await{
        Ok(t) => t,
        Err(e) => panic!("Error: {}", e),
    };
    println!("{:?}", &result);
}