pub mod vscodemarketplace;

use anyhow::Result;
use tempfile::tempdir;


use std::{
    fs::{self, File},
    process::Command,
};

pub fn get_hash(url: &str) -> Result<String> {
    // Create a temporary directoty inside of the directory returned by `std::env::temp_dir()`
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("temp.vsix");
    File::create(&file_path)?;
    
    smol::block_on(async {
        let mut resp = surf::get(url)
            .await
            .expect("failed to download file from url");
        let content: Vec<u8> = resp
            .body_bytes()
            .await
            .expect("failed to write bytes to file");

        fs::write(&file_path, content).expect("Unable to write file");
    });
    
    //let hash = digest_file(&file_path).expect("Failed to get sha256 hash");
    let nix_hash_process = Command::new("nix-hash")
        .arg("--flat")
        .arg("--base32")
        .arg("--type")
        .arg("sha256")
        .arg(&file_path.to_str().unwrap())
        .output()
        .unwrap();

    let hash = String::from_utf8(nix_hash_process.stdout)?;

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    //drop(input_file);
    temp_dir.close()?;

    Ok(hash)
}
