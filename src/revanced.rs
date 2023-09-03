use octocrab::models::repos::Release;

pub async fn search() {
    eprintln!("{:#?}", get_latest_version().await)
}

async fn get_latest_version() -> Release {
    let octocrab = octocrab::instance();

    octocrab
        .repos("revanced", "revanced-patches")
        .releases()
        .get_latest()
        .await
        .expect("Can't find the latest version of Revanced")
}
