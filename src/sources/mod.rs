pub mod vscodemarketplace;

use anyhow::Result;
use tempfile::Builder;

use std::{fs::File, io::copy, process::Command};

pub async fn get_hash(url: &str) -> Result<String> {
    // Create a temporary directoty inside of the directory returned by `std::env::temp_dir()`
    let tmp_dir = Builder::new().prefix(env!("CARGO_PKG_NAME")).tempdir()?;

    let response = reqwest::get(url).await?;

    let (mut dest_file, dest_path) = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.vsix");

        let fname = tmp_dir.path().join(fname);
        (File::create(&fname)?, fname)
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest_file)?;

    let nix_hash_process = Command::new("nix-hash")
        .arg("--flat")
        .arg("--base32")
        .arg("--type")
        .arg("sha256")
        .arg(&dest_path.to_str().unwrap())
        .output()
        .unwrap();

    let hash = String::from_utf8(nix_hash_process.stdout)?;

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    //drop(input_file);
    tmp_dir.close()?;

    Ok(hash)
}
