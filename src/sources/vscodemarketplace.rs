use serde_derive::{Deserialize, Serialize};

use anyhow::{anyhow, Result};
use surf::http::mime::JSON;

use crate::{
  package::{NixPackage, PackageKind},
  sources::get_hash
};

const EXT_QUERY_ADDRESS: &str =
    "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub filters: Vec<PayloadCriteria>,
    pub assetTypes: Vec<String>,
    pub flags: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadCriteria {
    pub criteria: Vec<PayloadCriterion>,
    pub pageNumber: u64,
    pub pageSize: u64,
    pub sortBy: u64,
    pub sortOrder: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadCriterion {
    pub filterType: u64,
    pub value: String,
}

impl Payload {
    pub fn new(unique_id: &str) -> Self {
        Payload {
            filters: vec![PayloadCriteria {
                criteria: vec![
                    PayloadCriterion {
                        filterType: 8,
                        value: String::from("Microsoft.VisualStudio.Code"),
                    },
                    PayloadCriterion {
                        filterType: 10,
                        value: unique_id.to_string(),
                    },
                    PayloadCriterion {
                        filterType: 12,
                        value: String::from("4096"),
                    },
                ],
                pageNumber: 1,
                pageSize: 2,
                sortBy: 0,
                sortOrder: 0,
            }],
            assetTypes: Vec::with_capacity(1),
            flags: 946,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VSMarketPlaceQueryResultResponse {
    pub results: Vec<VSMarketPlaceQueryResults>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceQueryResults {
    pub extensions: Vec<VSMarketPlaceExtension>,
    pub pagingToken: Option<String>,
    pub resultMetadata: Vec<VSMarketPlaceQueryResultMetaData>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceExtension {
    pub publisher: VSMarketPlaceExtensionPublisher,
    pub extensionId: String,
    pub extensionName: String,
    pub displayName: String,
    pub flags: String,
    pub lastUpdated: String,
    pub publishedDate: String,
    pub releaseDate: String,
    pub shortDescription: String,
    pub versions: Vec<VSMarketPlaceExtensionVersion>,
    pub statistics: Vec<VSMarketPlaceExtensionStatistic>,
    pub deploymentType: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceExtensionPublisher {
    pub publisherId: String,
    pub publisherName: String,
    pub displayName: String,
    pub flags: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceExtensionVersion {
    pub version: String,
    pub flags: String,
    pub lastUpdated: String,
    pub files: Vec<VSMarketPlaceExtensionVersionFile>,
    pub properties: Vec<VSMarketPlaceExtensionVersionProperty>,
    pub assetUri: String,
    pub fallbackAssetUri: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceExtensionVersionFile {
    pub assetType: String,
    pub source: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceExtensionVersionProperty {
    pub key: String,
    pub value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceExtensionStatistic {
    pub statisticName: String,
    pub value: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceQueryResultMetaData {
    pub metadataType: String,
    pub metadataItems: Vec<VSMarketPlaceQueryResultMetaDataItem>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSMarketPlaceQueryResultMetaDataItem {
    pub name: String,
    pub count: u64,
}

impl VSMarketPlaceQueryResultResponse {
  #[allow(dead_code)]
    pub async fn new(extension: &str) -> Result<Self> {
        smol::block_on(async {
            let data = Payload::new(&extension);
            let mut response = surf::post(EXT_QUERY_ADDRESS)
                .header("Accept", "application/json;api-version=6.1-preview.1")
                .header("Referer", "")
                .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Code/1.51.0 Chrome/83.0.4103.122 Electron/9.3.3 Safari/537.36")
                .header("X-Market-Client-Id", "VSCode 1.51.0")
                .content_type(JSON)
                .body(serde_json::to_string(&data)?)
                .send()
                .await.unwrap();

            if response.status() == 200 {
                let body: String = response.body_string().await.unwrap();
                let bdy = serde_json::from_str::<VSMarketPlaceQueryResultResponse>(&body).unwrap();
                Ok(bdy)
            } else {
                Err(anyhow!(
                    "Status: {}\nFailed to get version or extension is does not exist",
                    response.status()
                ))
            }
        })
    }
}

impl VSMarketPlaceExtension {
  pub async fn new(extension: &str) -> Result<Self> {
    smol::block_on(async {
      let data = Payload::new(&extension);
      let mut response = surf::post(EXT_QUERY_ADDRESS)
        .header("Accept", "application/json;api-version=6.1-preview.1")
        .header("Referer", "")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Code/1.51.0 Chrome/83.0.4103.122 Electron/9.3.3 Safari/537.36")
        .header("X-Market-Client-Id", "VSCode 1.51.0")
        .content_type(JSON)
        .body(serde_json::to_string(&data)?)
        .send()
        .await.expect("Error sending data");

      if response.status() == 200 {
        let response_body: String = response.body_string().await.unwrap();
        let response_json =
            serde_json::from_str::<VSMarketPlaceQueryResultResponse>(&response_body).unwrap();
        if !response_json.results.is_empty()
            && !response_json.results[0].extensions.is_empty()
        {
            Ok(response_json.results[0].extensions[0].clone())
        } else {
            Err(anyhow!("No results in VS Marektplace for extension"))
        }
      } else {
        Err(anyhow!(
            "Status: {}\nFailed to get version or extension is does not exist",
            response.status()
        ))
      }
    })
  }
}

impl From<VSMarketPlaceExtension> for NixPackage {
  fn from(ext: VSMarketPlaceExtension) -> Self {
    let publisher: String = ext.publisher.publisherName.to_string();
    let extension_name: String = ext.extensionName.to_string();
    let version: String = ext.versions[0].version.clone();

    let mut vsix_url: Box<String> = Box::new(String::from(""));

    for file_info in ext.versions[0].files.clone() {
      if file_info.assetType == "Microsoft.VisualStudio.Services.VSIXPackage" {
        *vsix_url = file_info.source;
      }
    }
    let sha256 = get_hash(&ext.versions[0].files[6].source).unwrap();

    NixPackage {
      kind: PackageKind::VscodeExtension{
        publisher: publisher.to_string(), 
        extension_name: extension_name .to_string()
      },
      name: format!("{}.{}", &publisher, &extension_name),
      pname: extension_name.to_string(),
      src: format!("https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/${publisher}/extension/{extName}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage", publisher=&publisher, extName=&extension_name, version=&version),
      version,
      sha256
    }
  }
}

/*
Extend
description = ext.shortDescription;
homepage = "https://marketplace.visualstudio.com/items?itemName={publisher}.{extension_name}";
license = // Could be take from gihub repo with licenses; [ unlicense /* or */ mit ];


*/

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;
    #[test]
    fn test_vscode() {
        smol::block_on(async {
            let expected: VSMarketPlaceExtension = serde_json::from_value(json!({
          "publisher": {
            "publisherId": "676a77c3-4b25-4793-af44-32acc176c330",
            "publisherName": "cometeer",
            "displayName": "cometeer",
            "flags": "verified"
          },
          "extensionId": "5377d680-e3f1-43bc-a2a8-0386b693b58b",
          "extensionName": "spacemacs",
          "displayName": "Spacemacs",
          "flags": "validated, public",
          "lastUpdated": "2017-10-05T10:10:51.8Z",
          "publishedDate": "2017-06-06T15:36:54.117Z",
          "releaseDate": "2017-06-06T15:36:54.117Z",
          "shortDescription": "Spacemacs themes for Visual Studio Code",
          "versions": [
            {
              "version": "1.1.1",
              "flags": "validated",
              "lastUpdated": "2017-10-05T10:10:52.033Z",
              "files": [
                {
                  "assetType": "Microsoft.VisualStudio.Code.Manifest",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Code.Manifest"
                },
                {
                  "assetType": "Microsoft.VisualStudio.Services.Content.Changelog",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.Content.Changelog"
                },
                {
                  "assetType": "Microsoft.VisualStudio.Services.Content.Details",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.Content.Details"
                },
                {
                  "assetType": "Microsoft.VisualStudio.Services.Icons.Default",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.Icons.Default"
                },
                {
                  "assetType": "Microsoft.VisualStudio.Services.Icons.Small",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.Icons.Small"
                },
                {
                  "assetType": "Microsoft.VisualStudio.Services.VsixManifest",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.VsixManifest"
                },
                {
                  "assetType": "Microsoft.VisualStudio.Services.VSIXPackage",
                  "source": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.VSIXPackage"
                }
              ],
              "properties": [
                {
                  "key": "Microsoft.VisualStudio.Services.Links.Repository",
                  "value": "git+https://github.com/cometeer/spacemacs-vscode.git"
                },
                {
                  "key": "Microsoft.VisualStudio.Services.Links.Getstarted",
                  "value": "git+https://github.com/cometeer/spacemacs-vscode.git"
                },
                {
                  "key": "Microsoft.VisualStudio.Services.Links.Support",
                  "value": "https://github.com/cometeer/spacemacs-vscode/issues"
                },
                {
                  "key": "Microsoft.VisualStudio.Services.Links.Learn",
                  "value": "https://github.com/cometeer/spacemacs-vscode#readme"
                },
                {
                  "key": "Microsoft.VisualStudio.Services.Links.Source",
                  "value": "git+https://github.com/cometeer/spacemacs-vscode.git"
                },
                {
                  "key": "Microsoft.VisualStudio.Code.Engine",
                  "value": "^1.12.0"
                },
                {
                  "key": "Microsoft.VisualStudio.Services.GitHubFlavoredMarkdown",
                  "value": "true"
                },
                {
                  "key": "Microsoft.VisualStudio.Code.ExtensionDependencies",
                  "value": ""
                }
              ],
              "assetUri": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877",
              "fallbackAssetUri": "https://cometeer.gallery.vsassets.io/_apis/public/gallery/publisher/cometeer/extension/spacemacs/1.1.1/assetbyname"
            }
          ],
          "statistics": [
            {
              "statisticName": "install",
              "value": 47541.0
            },
            {
              "statisticName": "averagerating",
              "value": 4.857142925262451
            },
            {
              "statisticName": "ratingcount",
              "value": 7.0
            },
            {
              "statisticName": "trendingdaily",
              "value": 0.002103713053539497
            },
            {
              "statisticName": "trendingmonthly",
              "value": 1.1254864836436311
            },
            {
              "statisticName": "trendingweekly",
              "value": 0.2208898706216472
            },
            {
              "statisticName": "updateCount",
              "value": 9334.0
            },
            {
              "statisticName": "weightedRating",
              "value": 4.590120457054699
            },
            {
              "statisticName": "downloadCount",
              "value": 66.0
            }
          ],
          "deploymentType": 0
  })).unwrap();

            let pre_actual = VSMarketPlaceQueryResultResponse::new("cometeer.spacemacs")
                .await
                .unwrap();

            let actual = pre_actual.results[0].extensions[0].clone();

            assert_eq!(actual.extensionId, expected.extensionId)
        });
    }
}
