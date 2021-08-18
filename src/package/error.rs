use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackageError {
    #[error("Config does not exist does not exist")]
    ConfigNotExist,
    #[error("Config is not a file")]
    ConfigNotFile,
    #[error("Invalid toml: {0:?}")]
    ConfigIsInvalidToml(#[from] toml::de::Error),
    #[error("I/O error: {0:?}")]
    ConfigIo(#[from] io::Error),
    #[error(
        "Config appears to be a file you are able to access, here is the metadata to debug {0:?}"
    )]
    ConfigUnknownError(fs::Metadata),
    #[error("Unable to get package")]
    DownloadPackageError,
}
