use crate::utils;
use octocrab::models::repos::{Asset, Release};
use regex::Regex;
use semver::Version;
use std::fs;

#[derive(Clone, Debug)]
enum RevancedRepo {
    Patches,
    Cli,
    Integrations,
}

impl RevancedRepo {
    fn owner(&self) -> &str {
        "ReVanced"
    }

    fn repo(&self) -> &str {
        match self {
            Self::Patches => "revanced-patches",
            Self::Cli => "revanced-cli",
            Self::Integrations => "revanced-integrations",
        }
    }

    fn targeted_ext(&self) -> &str {
        match self {
            RevancedRepo::Patches => ".jar",
            RevancedRepo::Cli => "-all.jar",
            RevancedRepo::Integrations => ".apk",
        }
    }
}

/// Revanced worker
pub async fn worker() {
    // Download patches if needed
    if let Some(patches_url) = search(&RevancedRepo::Patches).await {
        utils::download_file(patches_url).await;
    }

    // Download CLI if needed
    if let Some(cli_url) = search(&RevancedRepo::Cli).await {
        utils::download_file(cli_url).await;
    }

    // Download Integrations if needed
    if let Some(integrations_url) = search(&RevancedRepo::Integrations).await {
        utils::download_file(integrations_url).await;
    }
}

/// Search for the latest version
/// Returns option if update has been found with the right URL
async fn search(repo_type: &RevancedRepo) -> Option<String> {
    let github_latest_version = get_latest_version(repo_type).await;

    let latest_version = {
        let mut tag = github_latest_version.tag_name.chars();
        tag.next();
        Version::parse(tag.as_str())
            .unwrap_or_else(|_| panic!("Can't parse the latest version of {:?}.", repo_type))
    };

    if let Some(current_version) = get_current_version(repo_type) {
        // No need to update
        if latest_version <= current_version {
            return None;
        }
    }

    // Either no current version, or version outdated, we need an update
    // Fetching the URL to the latest version
    github_latest_version
        .assets
        .iter()
        .filter(|ele| {
            ele.browser_download_url
                .path()
                .ends_with(repo_type.targeted_ext())
        })
        .collect::<Vec<&Asset>>()
        .first()
        .map(|asset| {
            format!(
                "{}://{}{}",
                asset.browser_download_url.scheme(),
                asset
                    .browser_download_url
                    .host()
                    .expect("Can't get asset host."),
                asset.browser_download_url.path()
            )
        })
}

/// Grab the latest patches version published by ReVanced
async fn get_latest_version(repo: &RevancedRepo) -> Release {
    let octocrab = octocrab::instance();

    octocrab
        .repos(repo.owner(), repo.repo())
        .releases()
        .get_latest()
        .await
        .unwrap_or_else(|_| panic!("Can't find the latest version of {}.", repo.repo()))
}

/// Find the current version already downloaded
fn get_current_version(repo_type: &RevancedRepo) -> Option<semver::Version> {
    match fs::read_dir(utils::get_data_directory()) {
        Err(why) => {
            eprintln!("Error reading directory: {:?}", why.kind());
            None
        }
        Ok(paths) => {
            for path in paths {
                let re = Regex::new(&format!(
                    r"{}-(?P<version>\d+\.\d+\.\d+)\{}",
                    repo_type.repo(),
                    repo_type.targeted_ext()
                ))
                .unwrap_or_else(|_| panic!("Can't build regex formula for {:?}.", repo_type));
                if let Some(caps) = re.captures(
                    &path
                        .unwrap_or_else(|_| panic!("Can't match patterns for {:?}.", repo_type))
                        .path()
                        .display()
                        .to_string(),
                ) {
                    return Some(Version::parse(&caps["version"]).unwrap_or_else(|_| {
                        panic!("No version found in the asset of {:?}.", repo_type)
                    }));
                }
            }
            None
        }
    }
}
