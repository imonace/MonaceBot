use minidom::Element;
use teloxide::utils::markdown;

//const OBS_API_URL: &str = "https://api.opensuse.org/";
const OBS_API_BASE: &str = r#"https://api.opensuse.org/search/published/binary/id?match=@name=""#;
const OBS_API_REST: &str = r#"" and (contains-ic(@arch, "x86_64") or contains-ic(@arch, "noarch")) and not(contains-ic(@project, "home:")) and contains-ic(@baseproject, "openSUSE")"#;

pub async fn get_pkg(pkgname: String) -> String {
    let pkgname = pkgname.trim();
    // todo: add ascii filtering support
    if pkgname.is_empty() {
        "No pkgname provided\\.".to_string()
    } else {
        let query_result = query_pkg(&pkgname).await.unwrap();
        let version = format_pkg(&query_result);
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

fn format_pkg(query_result: &str) -> String {
    let mut query_result_mod = String::from(query_result);  // minidom 库禁止不带namespace的dom，强行 hack 一手
    query_result_mod.insert_str(query_result.find('\n').unwrap()-1, r#" xmlns="""#);
    let root: Element = query_result_mod.parse().unwrap();
    //println!("{:#?}",root);
    for child in root.children() {
        if child.attr("project") == Some("openSUSE:Factory") {
            let version = child.attr("version").unwrap();
            return version.to_string();
        }
    }
    "No version found\\.".to_string()
}
