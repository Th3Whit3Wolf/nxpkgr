mod package;
mod sources;
//mod openvsx;
mod extensions;

use anyhow::Result;
use package::NixPackage;

/*
    for reading json AST
    https://astexplorer.net
*/

#[tokio::main]
async fn main() -> Result<()> {
    let extensions: Vec<NixPackage> = extensions::get_extensions().await.unwrap();
    for ext in extensions {
        println!("{:#?}", ext);
    }

    Ok(())
}
