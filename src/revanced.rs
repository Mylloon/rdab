use crate::utils;
use octocrab::models::repos::{Asset, Release};
use regex::Regex;
use semver::Version;
use std::fs;

/// Revanced worker
pub async fn worker() {
    if let Some(patches_url) = search().await {
        // Download
        println!("TODO // Download {}", patches_url);
    }
}

/// Search for the latest version
/// Returns option if update has been found with the right URL
async fn search() -> Option<String> {
    let github_latest_version = get_latest_version().await;

    let latest_version = {
        let mut tag = github_latest_version.tag_name.chars();
        tag.next();
        Version::parse(tag.as_str()).unwrap()
    };

    if let Some(current_version) = get_current_version() {
        // No need to update
        if latest_version <= current_version {
            return None;
        }
    }

    // Either no current version, or version outdated, we need an update
    github_latest_version
        .assets
        .iter()
        .filter(|ele| ele.browser_download_url.path().ends_with(".jar"))
        .collect::<Vec<&Asset>>()
        .first()
        .map(|asset| {
            format!(
                "{}://{}{}",
                asset.browser_download_url.scheme(),
                asset.browser_download_url.host().unwrap(),
                asset.browser_download_url.path()
            )
        })
}

/// Grab the latest patches version published by ReVanced
async fn get_latest_version() -> Release {
    let octocrab = octocrab::instance();

    octocrab
        .repos("revanced", "revanced-patches")
        .releases()
        .get_latest()
        .await
        .expect("Can't find the latest version of Revanced")
}

/// Find the current version already downloaded
fn get_current_version() -> Option<semver::Version> {
    match fs::read_dir(utils::get_data_directory()) {
        Err(why) => {
            eprintln!("Error reading directory: {:?}", why.kind());
            None
        }
        Ok(paths) => {
            for path in paths {
                let re = Regex::new(r"revanced-patches-(?P<version>\d+\.\d+\.\d+)\.jar").unwrap();
                if let Some(caps) = re.captures(&path.unwrap().path().display().to_string()) {
                    return Some(Version::parse(&caps["version"]).unwrap());
                }
            }
            None
        }
    }
}
