use color_eyre::{
    eyre::{eyre, Report, Result, WrapErr},
    Section,
};

use serde::{Deserialize, Serialize};

use toml::Value;
use tracing::instrument;

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use crate::nix::NixPackage;

use super::{
    error::PackageError, github::TomlGitHubPackage, openvsx::TomlOpenVSXPackage,
    settings::TomlSettings, vsmarketplace::TomlVSCodeMarketPlacePackage,
};

use crate::sources::openvsx::OpenVSXExtension;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Overlay {
    #[serde(rename(deserialize = "vscode"))]
    VSCode,
    #[serde(rename(deserialize = "Vim"))]
    Vim,
    #[serde(rename(deserialize = "none"))]
    None,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlManifest {
    pub settings: Option<Box<TomlSettings>>,
    pub github: Option<BTreeMap<String, TomlGitHubPackage>>,
    pub openvsx: Option<BTreeMap<String, TomlOpenVSXPackage>>,
    pub vsmarketplace: Option<BTreeMap<String, TomlVSCodeMarketPlacePackage>>,
    #[serde(flatten)]
    pub other: BTreeMap<String, Value>,
}

#[instrument]
fn config_path_to_string(config_toml: PathBuf) -> String {
    match config_toml.to_str() {
        Some(valid_unicode) => valid_unicode.to_string(),
        None => config_toml.to_string_lossy().into_owned(),
    }
}

/*
 * Github Rate Limit
 * - Unauthenticated =>     60 requests per hour
 * - Authenticated   =>  5,000 requests per hour
 * - Enterprise      => 15,000 requests per hour
 */

impl TomlManifest {
    /// Read config file and all of its packages
    #[instrument]
    pub fn from_str(toml: &str) -> Result<Self, Report> {
        match toml::from_str(toml) {
            Ok(manifest) => Ok(manifest),
            Err(e) => Err(eyre!("Invalid toml"))
                .error(PackageError::ConfigIsInvalidToml(e))
                .suggestion(
                    "You can use https://www.toml-lint.com/ to help make sure your config is valid",
                ),
        }
    }

    #[instrument]
    pub fn from_file(config_toml: PathBuf) -> Result<Self, Report> {
        let config_path = config_path_to_string(config_toml.clone());
        let config_toml_as_path = Path::new(&config_path);
        if config_toml_as_path.is_file() {
            match fs::metadata(config_toml) {
                Ok(_meta) => match fs::read_to_string(config_toml_as_path) {
                    Ok(toml_file) => match TomlManifest::from_str(&toml_file) {
                        Ok(manifest) => Ok(manifest),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(eyre!("Config.toml: {}", config_path))
                        .error(PackageError::ConfigIo(e))
                        .suggestion("Make sure you have permisions to this file"),
                },
                Err(e) => Err(eyre!("Config.toml: {}", config_path))
                    .error(PackageError::ConfigIo(e))
                    .suggestion("Make sure you have permisions to this file"),
            }
        } else if config_toml.exists() {
            Err(eyre!("{}", config_path))
                .error(PackageError::ConfigNotFile)
                .suggestion("Make sure path is to a file")
        } else {
            Err(eyre!("{}", config_path))
                .error(PackageError::ConfigNotExist)
                .suggestion("Make sure you are using a real path to a file")
        }
    }

    /*
        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct DetailedTomlOpenVSXPackage {
        src: String,
        pin: Option<String>,
        extract: Option<String>,
        #[serde(default = "default_overlay")]
        overlay: Option<Overlay>,
        passthru: Option<Table>
    }
        */
    pub async fn get_openvsx_nixpkgs(self) -> Result<Vec<NixPackage>> {
        let mut openvsx: Vec<NixPackage> = Vec::new();
        if let Some(map) = self.openvsx {
            for (pname, openvsx_package) in map {
                match openvsx_package {
                    TomlOpenVSXPackage::Simple(unique_id) => {
                        match OpenVSXExtension::get(unique_id).await {
                            Ok(pkg) => openvsx.push(pkg.to_nixpkg(pname, None)),
                            Err(_) => {
                                return Err(eyre!("Unable to get package for {}", pname)
                                    .error(PackageError::DownloadPackageError))
                            }
                        }
                    }
                    TomlOpenVSXPackage::Detailed(details) => {
                        let pkg = if let Some(version) = details.pin {
                            match OpenVSXExtension::get_with_version(details.src, version).await {
                                Ok(p) => p,
                                Err(_) => {
                                    return Err(eyre!("Unable to get package for {}", pname)
                                        .error(PackageError::DownloadPackageError))
                                }
                            }
                        } else {
                            match OpenVSXExtension::get(details.src).await {
                                Ok(p) => p,
                                Err(_) => {
                                    return Err(eyre!("Unable to get package for {}", pname)
                                        .error(PackageError::DownloadPackageError))
                                }
                            }
                        };
                        openvsx.push(pkg.to_nixpkg(pname, details.extract))
                    }
                }
            }
            Ok(openvsx)
        } else {
            Err(eyre!("There are not openvsx packages"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn test() {}
}
