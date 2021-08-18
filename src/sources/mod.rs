pub mod github;
pub mod openvsx;
pub mod vscodemarketplace;

use pulldown_cmark::{Event, Options, Parser, Tag};
use tempfile::Builder;

use color_eyre::{
    eyre::{eyre, Report, WrapErr, Result},
    Section,
};

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

#[derive(PartialEq)]
enum ProgressLongDesc {
    LookingForMainHeader,
    FoundMainHeader,
    ReadingText,
    Done
}

pub async fn get_long_description(url: String) -> Result<String, Report> {
    let resp = reqwest::get(url).await?;

    let status = resp.status();

    if status.is_success() {
        let markdown = resp.text().await?;
        let mut long_description = String::new();
        let mut progress = ProgressLongDesc::LookingForMainHeader;
        let parser = Parser::new_ext(&markdown, Options::empty());
        
        for event in parser {
            match event {
                Event::Start(inner) => {
                    match inner {
                        Tag::Heading(_n) => {
                            if progress == ProgressLongDesc::LookingForMainHeader {
                                progress = ProgressLongDesc::FoundMainHeader;
                            } else if progress == ProgressLongDesc::ReadingText {
                                progress = ProgressLongDesc::Done;
                            }
                        }
                        Tag::Paragraph => {
                            if progress == ProgressLongDesc::FoundMainHeader {
                                progress = ProgressLongDesc::ReadingText; 
                            } 
                        },
                        _ => ()
                    }
                }
                Event::Text(cow_str) => {
                    if progress == ProgressLongDesc::ReadingText {
                        long_description.push_str(cow_str.into_string().as_str())
                    }
                },
                Event::SoftBreak => {
                    if progress == ProgressLongDesc::ReadingText {
                        long_description.push('\n')
                    }
                },
                Event::End(inner) => {
                    if inner == Tag::Paragraph && progress == ProgressLongDesc::ReadingText {
                        progress = ProgressLongDesc::Done;
                    }
                }
                _ => ()
            }

            if progress == ProgressLongDesc::Done {
                break
            }
        };

        Ok(long_description)
    } else if let Some(reason) = status.canonical_reason() {
        Err(eyre!("Recieved {}, while attempting to get meta.longDescription.", reason))
    } else {
        Err(eyre!("{}",  status.to_string()))
    }
}
