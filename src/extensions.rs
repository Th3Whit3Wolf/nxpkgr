use crate::{
    package::{NixPackage, Package, Sources},
    sources::vscodemarketplace::VSMarketPlaceExtension,
};

use anyhow::{anyhow, Context, Result};

use std::fs;
//use std::io::{BufReader, Read, Write};

const PATH_TO_EXAMPLE: &str = "./data/example_config.toml";

pub async fn get_extensions() -> Result<Vec<NixPackage>> {
    // Read a Toml file./
    if let Ok(toml_file) = fs::read_to_string(PATH_TO_EXAMPLE) {
        let mut nixpkgs: Vec<NixPackage> = Vec::new();
        let packages = Package::get_packages(toml_file);

        for package in packages {
            match package.src {
                Sources::VSCodeMarketPlace => {
                    let pkg = VSMarketPlaceExtension::new(&package.name)
                        .await
                        .with_context(|| format!("failed to parse {}", &package.name))?;
                    let nixpkg: NixPackage = pkg.into();
                    nixpkgs.push(nixpkg);
                }
                Sources::OpenVSX => (),
            }
        }
        Ok(nixpkgs)
    } else {
        Err(anyhow!("unable to read \"{}\" to string", PATH_TO_EXAMPLE))
    }
}
