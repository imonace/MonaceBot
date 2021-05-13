use chrono::Local;
use minidom::Element;
use std::fmt;
use teloxide::utils::markdown::escape;

const OBS_API_BASE: &str = r#"https://api.opensuse.org/search/published/binary/id?match=@name="#;
const OBS_API_ARCH: &str = r#" and (contains-ic(@arch, "x86_64") or contains-ic(@arch, "noarch")) and contains-ic(@baseproject, "openSUSE:")"#;
const OBS_API_PROJ: &str = r#" and not(contains-ic(@project, "home:")) and not(contains-ic(@project, "devel:"))"#;

struct PkgVersion {
    pkgname: String,
    tw_official: String,
    tw_experiment: String,
    leap_official: String,
    leap_experiment: String,
}

impl fmt::Display for PkgVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tw_official.is_empty()
            && self.tw_experiment.is_empty()
            && self.leap_official.is_empty()
            && self.leap_experiment.is_empty()
        {
            write!(f, "{}", escape("No official version founded."))
        } else {
            write!(
                f,
                "*Package*: {}\n{}\n*openSUSE Tumbleweed:*\n{}{}*openSUSE Leap 15\\.2:*\n{}{}",
                escape(&self.pkgname),
                escape("-------------------------"),
                self.tw_official,
                self.tw_experiment,
                self.leap_official,
                self.leap_experiment
            )
        }
    }
}

pub async fn get_pkg(pkgname: String) -> String {
    let pkgname = pkgname
        .trim()
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        .collect::<String>();
    log::info!("{}: Get pkg \"{}\" requested.", Local::now(), &pkgname);

    if pkgname.is_empty() {
        escape("No pkgname provided.")
    } else {
        let query_result = match query_pkg(&pkgname).await {
            Ok(t) => t,
            Err(_) => return escape("An error occurred during requesting."),
        };
        let version = match format_pkg(&pkgname, &query_result) {
            Ok(t) => t,
            Err(_) => return escape("An error occurred during parsing."),
        };
        version.to_string()
    }
}

async fn query_pkg(pkgname: &str) -> Result<String, reqwest::Error> {
    let obs_username: String =
        std::env::var("OBS_USERNAME").expect("OBS_USERNAME env variable not found.");
    let obs_password: String =
        std::env::var("OBS_PASSWORD").expect("OBS_PASSWORD env variable not found.");

    let client = reqwest::Client::new();
    let url = format!("{}\"{}\"{}{}", OBS_API_BASE, pkgname, OBS_API_ARCH, OBS_API_PROJ);

    Ok(client
        .get(url)
        .basic_auth(obs_username, Some(obs_password))
        .send()
        .await?
        .text()
        .await?)
}

fn format_pkg(pkgname: &str, query_result: &str) -> Result<PkgVersion, minidom::Error> {
    let mut xml = String::from(query_result);
    xml.insert_str(xml.find('\n').unwrap() - 1, r#" xmlns="""#);

    let mut tw_official = String::new();
    let mut tw_experiment = String::new();
    let mut leap_official = String::new();
    let mut leap_experiment = String::new();
    let mut patchinfo = 0;

    let root: Element = xml.parse()?;
    for child in root.children() {
        if child.attr("project") == Some("openSUSE:Factory") {
            tw_official = escape(&format!(
                " - official:\n    - {}-{}\n",
                child.attr("version").unwrap(),
                child.attr("release").unwrap()
            ));
        } else if child.attr("repository") == Some("openSUSE_Tumbleweed") {
            tw_experiment += &escape(&format!(
                " - {}:\n    - {}-{}\n",
                child.attr("project").unwrap(),
                child.attr("version").unwrap(),
                child.attr("release").unwrap()
            ));
        } else if child.attr("project") == Some("openSUSE:Leap:15.2:Update") {
            let patchinfo_new = child
                .attr("package")
                .unwrap()
                .split('.')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            if patchinfo_new > patchinfo {
                patchinfo = patchinfo_new;
                leap_official = escape(&format!(
                    " - official:\n    - {}-{}\n",
                    child.attr("version").unwrap(),
                    child.attr("release").unwrap()
                ));
            };
        } else if child.attr("project") == Some("openSUSE:Leap:15.2") && patchinfo == 0 {
            leap_official = escape(&format!(
                " - official:\n    - {}-{}\n",
                child.attr("version").unwrap(),
                child.attr("release").unwrap()
            ));
        } else if child.attr("repository") == Some("openSUSE_Leap_15.2") {
            leap_experiment += &escape(&format!(
                " - {}:\n    - {}-{}\n",
                child.attr("project").unwrap(),
                child.attr("version").unwrap(),
                child.attr("release").unwrap()
            ));
        }
    }

    Ok(PkgVersion {
        pkgname: pkgname.to_string(),
        tw_official,
        tw_experiment,
        leap_official,
        leap_experiment,
    })
}
