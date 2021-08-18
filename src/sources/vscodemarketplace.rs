use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, REFERER, USER_AGENT,
};
use tokio::{runtime::Handle, task};

use crate::{
    nix::{NixLicense, NixPackage, NixPackageMeta},
    sources::{get_hash, get_long_description},
};

const EXT_QUERY_ADDRESS: &str =
    "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub filters: Vec<PayloadCriteria>,
    pub asset_types: Vec<String>,
    pub flags: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadCriteria {
    pub criteria: Vec<PayloadCriterion>,
    pub page_number: u64,
    pub page_size: u64,
    pub sort_by: u64,
    pub sort_order: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadCriterion {
    pub filter_type: u64,
    pub value: String,
}

impl Payload {
    pub fn new(unique_id: &str) -> Self {
        Payload {
            filters: vec![PayloadCriteria {
                criteria: vec![
                    PayloadCriterion {
                        filter_type: 8,
                        value: String::from("Microsoft.VisualStudio.Code"),
                    },
                    PayloadCriterion {
                        filter_type: 10,
                        value: unique_id.to_string(),
                    },
                    PayloadCriterion {
                        filter_type: 12,
                        value: String::from("4096"),
                    },
                ],
                page_number: 1,
                page_size: 2,
                sort_by: 0,
                sort_order: 0,
            }],
            asset_types: Vec::with_capacity(1),
            flags: 946,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VSMarketPlaceQueryResultResponse {
    pub results: Vec<VSMarketPlaceQueryResults>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceQueryResults {
    pub extensions: Vec<VSMarketPlaceExtension>,
    pub paging_token: Option<String>,
    pub result_metadata: Vec<VSMarketPlaceQueryResultMetaData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtension {
    pub publisher: VSMarketPlaceExtensionPublisher,
    pub extension_id: String,
    pub extension_name: String,
    pub display_name: String,
    pub flags: String,
    pub last_updated: String,
    pub published_date: String,
    pub release_date: String,
    pub short_description: String,
    pub versions: Vec<VSMarketPlaceExtensionVersion>,
    pub statistics: Vec<VSMarketPlaceExtensionStatistic>,
    pub deployment_type: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionPublisher {
    pub publisher_id: String,
    pub publisher_name: String,
    pub display_name: String,
    pub flags: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionVersion {
    pub version: String,
    pub flags: String,
    pub last_updated: String,
    pub files: Vec<VSMarketPlaceExtensionVersionFile>,
    pub properties: Vec<VSMarketPlaceExtensionVersionProperty>,
    pub asset_uri: String,
    pub fallback_asset_uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionVersionFile {
    pub asset_type: AssetTypeMicrosoftVisualStudio,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AssetTypeMicrosoftVisualStudio {
    #[serde(rename = "Microsoft.VisualStudio.Code.Manifest")]
    CodeManifest,
    #[serde(rename = "Microsoft.VisualStudio.Services.Content.Changelog")]
    ServicesContentChangelog,
    #[serde(rename = "Microsoft.VisualStudio.Services.Content.Details")]
    ServicesContentDetails,
    #[serde(rename = "Microsoft.VisualStudio.Services.Content.License")]
    ServicesContentLicense,
    #[serde(rename = "Microsoft.VisualStudio.Services.Icons.Default")]
    ServicesIconsDefault,
    #[serde(rename = "Microsoft.VisualStudio.Services.Icons.Small")]
    ServicesIconsSmall,
    #[serde(rename = "Microsoft.VisualStudio.Services.VsixManifest")]
    ServicesVsixManifest,
    #[serde(rename = "Microsoft.VisualStudio.Services.VSIXPackage")]
    ServicesVSIXPackage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VSMarketPlaceExtensionVersionProperty {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionStatistic {
    pub statistic_name: String,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceQueryResultMetaData {
    pub metadata_type: String,
    pub metadata_items: Vec<VSMarketPlaceQueryResultMetaDataItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VSMarketPlaceQueryResultMetaDataItem {
    pub name: String,
    pub count: u64,
}

impl VSMarketPlaceQueryResultResponse {
    #[allow(dead_code)]
    pub async fn get(extension: &str) -> Result<Self> {
        let data = Payload::new(extension);

        let mut headers = HeaderMap::new();

        // Declare headers
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json;api-version=6.1-preview.1"),
        );
        headers.insert(REFERER, HeaderValue::from_static(""));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Code/1.51.0 Chrome/83.0.4103.122 Electron/9.3.3 Safari/537.36"));
        headers.insert(
            HeaderName::from_static("x-market-client-id"),
            HeaderValue::from_static("VSCode 1.51.0"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response: VSMarketPlaceQueryResultResponse = reqwest::Client::new()
            .post(EXT_QUERY_ADDRESS)
            .headers(headers)
            .json(&data)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

impl VSMarketPlaceExtension {
    pub async fn get(extension: &str) -> Result<Self> {
        let query: VSMarketPlaceQueryResultResponse =
            VSMarketPlaceQueryResultResponse::get(extension).await?;

        if !query.results.is_empty() && !query.results[0].extensions.is_empty() {
            Ok(query.results[0].extensions[0].clone())
        } else {
            Err(anyhow!("No results in VS Marektplace for extension"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ManifestInfo {
    homepage: Option<String>,
    github: Option<String>,
    source: Option<String>,
}

async fn get_manifest_info(files: &Vec<VSMarketPlaceExtensionVersionFile>) -> Option<ManifestInfo> {
    let mut manifest_vec = Vec::new();
    let mut homepage_box = Box::new(String::from(""));
    let mut github_box = Box::new(String::from(""));
    let mut source_box = Box::new(String::from(""));

    for f in files {
        if f.asset_type == AssetTypeMicrosoftVisualStudio::CodeManifest {
            manifest_vec.push(
                reqwest::get(&f.source)
                    .await
                    .expect("Error: unable to connect to url")
                    .text()
                    .await
                    .expect("Error: unable to get response body"),
            )
        }
    }

    if !manifest_vec.is_empty() {
        if let Ok(doc) = roxmltree::Document::parse(&manifest_vec[0]) {
            for node in doc.descendants() {
                if node.is_element() {
                    if node.has_tag_name("Property") {
                        if let Some(property_id) = node.attribute_node("Id") {
                            match property_id.value() {
                                "Microsoft.VisualStudio.Services.Links.Learn" => {
                                    if let Some(property_value) = node.attribute_node("Value") {
                                        *homepage_box = property_value.value().to_string()
                                    }
                                }
                                "Microsoft.VisualStudio.Services.Links.GitHub" => {
                                    if let Some(property_value) = node.attribute_node("Value") {
                                        *github_box = property_value.value().to_string()
                                    }
                                }
                                "Microsoft.VisualStudio.Services.Links.Source" => {
                                    if let Some(property_value) = node.attribute_node("Value") {
                                        *source_box = property_value.value().to_string()
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }

            Some(ManifestInfo {
                homepage: if *homepage_box != "" {
                    Some(*homepage_box)
                } else {
                    None
                },
                github: if *github_box != "" {
                    Some(*github_box)
                } else {
                    None
                },
                source: if *source_box != "" {
                    Some(*source_box)
                } else {
                    None
                },
            })
        } else {
            None
        }
    } else {
        None
    }
}

async fn get_github_license(github_url: String) -> Option<&'static NixLicense> {
    let github = github_url.trim_end_matches(".git");
    let github = github.trim_start_matches("https://github.com/");
    let split_github_url: Vec<&str> = github.split('/').collect();
    let github_author = split_github_url[0];
    let github_repo = split_github_url[1];
    if let Some(lic) = octocrab::instance()
        .repos(github_author, github_repo)
        .license()
        .await
        .expect("Error: getting repo information")
        .license
    {
        NixLicense::from_str(&lic.name)
    } else {
        None
    }
}

impl From<VSMarketPlaceExtension> for NixPackage {
    fn from(ext: VSMarketPlaceExtension) -> Self {
        let publisher: String = ext.publisher.publisher_name.to_string();
        let extension_name: String = ext.extension_name.to_string();
        let version: String = ext.versions[0].version.clone();
        let src = format!("https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{extName}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage", publisher=&publisher, extName=&extension_name, version=&version);
        let src_clone = &src.to_string();

        let sha256: String = task::block_in_place(move || {
            Handle::current().block_on(async move {
                // do something async
                get_hash(src_clone)
                    .await
                    .expect("Error: unable to get hash of vsix")
            })
        });

        let description = if !&ext.short_description.is_empty() {
            Some(ext.short_description.to_string())
        } else {
            None
        };

        let mut long_description_box = Box::new(String::from(""));
        let mut changelog_box = Box::new(String::from(""));

        for f in &ext.versions[0].files.clone() {
            match f.asset_type {
                AssetTypeMicrosoftVisualStudio::ServicesContentChangelog => {
                    *changelog_box = f.source.to_string();
                }
                AssetTypeMicrosoftVisualStudio::ServicesContentDetails => {
                    *long_description_box = task::block_in_place(move || {
                        Handle::current().block_on(async move {
                            // do something async
                            get_long_description(f.source.clone())
                                .await
                                .expect("Error: unable to get readme of extension")
                        })
                    });
                }
                _ => (),
            }
        }

        let (license, homepage) = if let Some(manifest_info) = task::block_in_place(move || {
            Handle::current().block_on(async move {
                // do something async
                get_manifest_info(&ext.versions[0].files).await
            })
        }) {
            let license = if let Some(github_url) = manifest_info.github {
                if let Some(lic) = task::block_in_place(move || {
                    Handle::current().block_on(async move {
                        // do something async
                        get_github_license(github_url).await
                    })
                }) {
                    Some(vec![*lic])
                } else {
                    None
                }
            } else {
                None
            };
            let homepage = manifest_info.homepage;

            (license, homepage)
        } else {
            (None, None)
        };

        let long_description = if *long_description_box != "" {
            Some(*long_description_box)
        } else {
            None
        };

        let changelog = if *changelog_box != "" {
            Some(vec![*changelog_box])
        } else {
            None
        };
        let meta = NixPackageMeta {
            description,
            long_description,
            homepage,
            license,
            changelog,
            ..Default::default()
        };

        NixPackage {
            name: format!("{}.{}", &publisher, &extension_name),
            pname: extension_name,
            src,
            version,
            sha256,
            meta,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_vscode() {
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

        let actual: VSMarketPlaceExtension = VSMarketPlaceExtension::get("cometeer.spacemacs")
            .await
            .unwrap();
        assert_eq!(actual.publisher, expected.publisher);
        assert_eq!(actual.extension_id, expected.extension_id);
        assert_eq!(actual.extension_name, expected.extension_name);
        assert_eq!(actual.display_name, expected.display_name);
        assert_eq!(actual.flags, expected.flags);
        assert_eq!(actual.last_updated, expected.last_updated);
        assert_eq!(actual.published_date, expected.published_date);
        assert_eq!(actual.release_date, expected.release_date);
        assert_eq!(actual.short_description, expected.short_description);
        assert_eq!(actual.versions, expected.versions);
        assert_eq!(actual.deployment_type, expected.deployment_type);
    }
}
