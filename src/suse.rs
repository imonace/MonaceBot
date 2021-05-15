use chrono::Local;
use minidom::Element;
use std::fmt;
use teloxide::utils::html::bold;

const OBS_API_BASE: &str = r#"https://api.opensuse.org/search/published/binary/id?match=@name="#;
const OBS_API_ARCH: &str = r#" and (contains-ic(@arch, "x86_64") or contains-ic(@arch, "noarch")) and contains-ic(@baseproject, "openSUSE:")"#;
const OBS_API_PROJ: &str =
    r#" and not(contains-ic(@project, "home:")) and not(contains-ic(@project, "devel:"))"#;

#[derive(Debug)]
struct PkgVersion {
    pkgname: String,
    tw_off: String,
    tw_exp: String,
    lp_off: String,
    lp_exp: String,
}

impl fmt::Display for PkgVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tw_off.is_empty()
            && self.tw_exp.is_empty()
            && self.lp_off.is_empty()
            && self.lp_exp.is_empty()
        {
            write!(f, "No official version founded.")
        } else {
            write!(
                f,
                "{}: {}\n-------------------------\n{}:\n{}{}{}:\n{}{}",
                bold("Package"),
                self.pkgname,
                bold("openSUSE Tumbleweed"),
                self.tw_off,
                self.tw_exp,
                bold("openSUSE Leap 15.2"),
                self.lp_off,
                self.lp_exp
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
        "No pkgname provided.".to_string()
    } else {
        let query_result = match query_pkg(&pkgname).await {
            Ok(t) => t,
            Err(_) => return "An error occurred during requesting.".to_string(),
        };
        let version = match format_pkg(&pkgname, &query_result) {
            Ok(t) => t,
            Err(_) => return "An error occurred during parsing.".to_string(),
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
    let url = format!(
        "{}\"{}\"{}{}",
        OBS_API_BASE, pkgname, OBS_API_ARCH, OBS_API_PROJ
    );

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

    let mut tw_off = String::new();
    let mut tw_exp = String::new();
    let mut lp_off = String::new();
    let mut lp_exp = String::new();
    let mut patchinfo_rev = 0;

    let root: Element = xml.parse()?;
    for child in root.children() {
        let project = child.attr("project");
        let version = child.attr("version");
        let release = child.attr("release");
        let repository = child.attr("repository");

        if project == Some("openSUSE:Factory") {
            tw_off = format_version("official", version.unwrap(), release.unwrap());
        } else if repository == Some("openSUSE_Tumbleweed") {
            tw_exp += &format_version(project.unwrap(), version.unwrap(), release.unwrap());
        } else if project == Some("openSUSE:Leap:15.2:Update") {
            let patchinfo_new = child
                .attr("package")
                .unwrap()
                .split('.')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            if patchinfo_new > patchinfo_rev {
                patchinfo_rev = patchinfo_new;
                lp_off = format_version("official", version.unwrap(), release.unwrap());
            };
        } else if project == Some("openSUSE:Leap:15.2") && patchinfo_rev == 0 {
            lp_off = format_version("official", version.unwrap(), release.unwrap());
        } else if repository == Some("openSUSE_Leap_15.2") {
            lp_exp += &format_version(project.unwrap(), version.unwrap(), release.unwrap());
        }
    }

    Ok(PkgVersion {
        pkgname: pkgname.to_string(),
        tw_off,
        tw_exp,
        lp_off,
        lp_exp,
    })
}

fn format_version(project: &str, verison: &str, release: &str) -> String {
    format!(" - {}: {}-{}\n", project, verison, release)
}
