mod nix;
mod package;
mod sources;

use color_eyre::eyre;
use eyre::{Report, Result};
use tracing::instrument;

use std::path::{Path, PathBuf};

/*
    for reading json AST
    https://astexplorer.net
*/

const PATH_TO_EXAMPLE_TOML: &str = "./data/example_config.toml";

#[instrument]
#[tokio::main]
async fn main() -> Result<(), Report> {
    install_tracing();

    color_eyre::config::HookBuilder::default()
        .issue_url(concat!(env!("CARGO_PKG_HOMEPAGE"), "/issues/new"))
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .issue_filter(|kind| match kind {
            color_eyre::ErrorKind::NonRecoverable(_) => false,
            color_eyre::ErrorKind::Recoverable(_) => true,
        })
        .install()?;

    let manifest = package::TomlManifest::from_file(Path::new(PATH_TO_EXAMPLE_TOML).to_path_buf())?;
    let openvsx_packages = manifest.get_openvsx_nixpkgs().await?;
    println!("{:#?}", openvsx_packages);

    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
