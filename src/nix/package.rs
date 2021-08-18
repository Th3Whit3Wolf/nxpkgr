use color_eyre::{
    eyre::{eyre, Report, WrapErr},
    Section,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use std::path::{Path, PathBuf};

use crate::package::TomlManifest;

use super::{license::NixLicense, platforms::NixPlatforms};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct NixPackage {
    pub name: String,
    pub pname: String,
    pub src: String,
    pub version: String,
    pub sha256: String,
    pub meta: NixPackageMeta,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct NixPackageMeta {
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub branch: Option<String>,
    pub homepage: Option<String>,
    pub download_page: Option<String>,
    pub changelog: Option<Vec<String>>,
    pub license: Option<Vec<NixLicense>>,
    pub priority: Option<String>,
    pub maintainers: Option<String>,
    pub platforms: Option<NixPlatforms>,
    pub tests: Option<String>,
    pub timeout: Option<u64>,
    pub hydra_platforms: Option<String>,
    pub broken: Option<bool>,
    pub update_walker: Option<bool>,
}

impl Default for NixPackageMeta {
    fn default() -> Self {
        NixPackageMeta {
            description: None,
            long_description: None,
            branch: None,
            homepage: None,
            download_page: None,
            changelog: None,
            license: None,
            priority: None,
            maintainers: Some(String::from("th3whit3wolf")),
            platforms: Some(NixPlatforms::None),
            tests: None,
            timeout: None,
            hydra_platforms: None,
            broken: None,
            update_walker: None,
        }
    }
}
