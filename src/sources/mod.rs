pub mod github;
pub mod openvsx;
pub mod vscodemarketplace;

use anyhow::Result;
use pulldown_cmark::{Event, Options, Parser, Tag};
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

pub async fn get_long_description(url: String) -> Result<String> {
    let md = reqwest::get(url).await?.text().await?;
    let mut in_first_header = false;
    let mut long_description: Box<String> = Box::new(String::from(""));
    Parser::new_ext(&md, Options::all()).map(|event| match event {
        Event::Start(tag) => {
            if let Tag::Heading(num) = tag {
                if num < 4 {
                    in_first_header = true
                }
            }
        }
        Event::Text(text) => {
            if in_first_header && *long_description != String::from("") {
                *long_description = text.into_string();
            }
        }
        _ => (),
    });

    Ok(*long_description)
}
