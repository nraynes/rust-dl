use reqwest::Client;
use reqwest::blocking::Client as SyncClient;
use std::fs::File;
use std::io::{Cursor, copy};
use std::path::PathBuf;
use url::Url;

use crate::dl_error::DownloaderError;

pub fn download_sync(url: Url, path: PathBuf) -> Result<(), DownloaderError> {
    let client = SyncClient::new();
    let mut file = File::create(path)?;
    let response = client.get(url).send()?;
    let mut content = Cursor::new(response.bytes()?);
    copy(&mut content, &mut file)?;
    Ok(())
}

pub async fn download(url: Url, path: PathBuf) -> Result<(), DownloaderError> {
    let client = Client::new();
    let mut file = File::create(path)?;
    let response = client.get(url).send().await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use assert_fs::TempDir;

    use super::*;

    #[test]
    fn test_download_sync() {
        let temp = TempDir::new().unwrap();
        let pathway = temp.join(PathBuf::from("commit-msg"));
        download_sync(Url::parse("https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/arm-macos/commit-msg").unwrap(), pathway.clone()).unwrap();
        assert!(fs::exists(pathway).unwrap());
    }

    #[test]
    fn test_download() {}
}
