use serde::{Deserialize, Serialize};
use spin_sdk::http::IntoResponse;

pub const INFO_ENDPOINT: &str = "/info";
pub const AUTHORIZE_ENDPOINT: &str = "/authorize";
pub const DATA_ENDPOINT: &str = "/data";

#[derive(Serialize, Deserialize)]
struct PluginInfo {
    name: String,
    short_description: String,
    url: String,
    version: String,
    icon_url: String,
}

pub struct About {}

impl About {
    pub async fn generate_info(name: &str, short_description: &str, url: &str, version: &str, icon_url: &str) -> anyhow::Result<impl IntoResponse> {
        let plugin_info = PluginInfo {
            name: name.to_string(),
            short_description: short_description.to_string(),
            url: url.to_string(),
            version: version.to_string(),
            icon_url: icon_url.to_string(),
        };
        let j = serde_json::to_string(&plugin_info)?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK).body(j)?)
    }
}