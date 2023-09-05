use std::fs::File;
use std::io::Write;
use std::{env, path::PathBuf};

/// Data directory
pub fn get_data_directory() -> PathBuf {
    let pwd = env::current_dir().expect("Can't find the current directory.");

    pwd.join("data")
}

/// Download a file to data directory
pub async fn download_file(url: String) {
    let source = reqwest::Url::parse(&url).expect("Can't parse the URL.");
    let fname = source
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .expect("Can't find the filename of URL.");

    match reqwest::get(source.clone()).await {
        Ok(data) => match data.bytes().await {
            Ok(bytes) => {
                let filepath = get_data_directory().join(fname);

                // Create file
                let mut dest = File::create(&filepath)
                    .unwrap_or_else(|_| panic!("Can't create the file at {}.", filepath.display()));

                // Write data to the file
                dest.write_all(&bytes)
                    .unwrap_or_else(|_| panic!("Can't write to file at {}.", filepath.display()));
            }
            Err(..) => eprintln!("Can't download {}.", fname),
        },
        Err(..) => eprintln!("Can't get {}.", fname),
    }
}
