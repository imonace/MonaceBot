use minidom::Element;
use chrono::prelude::Local;
use teloxide::utils::markdown;

//const OBS_API_URL: &str = "https://api.opensuse.org/";
const OBS_API_BASE: &str = r#"https://api.opensuse.org/search/published/binary/id?match=@name=""#;
const OBS_API_REST: &str = r#"" and (contains-ic(@arch, "x86_64") or contains-ic(@arch, "noarch")) and not(contains-ic(@project, "home:")) and contains-ic(@baseproject, "openSUSE")"#;

pub async fn get_pkg(pkgname: String) -> String {
    let pkgname = pkgname.trim().chars().filter(|&c| c.is_ascii_alphanumeric() || c == '-').collect::<String>();
    log::info!("{}: Get pkg \"{}\" requested.", Local::now(), &pkgname);
    if pkgname.is_empty() {
        markdown::escape("No pkgname provided.")
    } else {
        let query_result = match query_pkg(&pkgname).await {
            Ok(t) => t,
            Err(_) => return markdown::escape("An error occurred during requesting."),
        };
        let version = match format_pkg(&query_result) {
            Ok(None) => return markdown::escape("No official version founded."),
            Ok(t) => t.unwrap(),
            Err(_) => return markdown::escape("An error occurred during parsing.")
        };
        let dash = "-------------------------";
        format!("*Package*: {}\n{}\n*openSUSE Tumbleweed:*\n    official: {}\n{}\nFunction under constructing\\!",
            markdown::escape(&pkgname), markdown::escape(&dash), markdown::escape(&version), markdown::escape(&dash))
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

fn format_pkg(query_result: &str) -> Result<Option<String>, minidom::Error> {
    let mut xml = String::from(query_result);
    xml.insert_str(xml.find('\n').unwrap()-1, r#" xmlns="""#);
    let root: Element = match xml.parse() {
        Ok(root) => root,
        Err(e) => return Err(e),
    };
    for child in root.children() {
        if child.attr("project") == Some("openSUSE:Factory") {
            let version = child.attr("version").unwrap().to_string();
            return Ok(Some(version));
        }
    }
    Ok(None)
}
