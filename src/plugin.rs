use serde::{Deserialize, Serialize};
use spin_sdk::http::IntoResponse;
use spin_sdk::http::StatusCode;

pub const INFO_ENDPOINT: &str = "/info";
pub const AUTHORIZE_ENDPOINT: &str = "/authorize";
pub const DATA_ENDPOINT: &str = "/data";

#[derive(Serialize, Deserialize)]
struct PluginInfo {
    identifier: String,
    name: String,
    short_description: String,
    url: String,
    version: String,
    icon_url: String,
    external_endpoints: Option<Vec<String>>,
    parameters: Option<Vec<Parameter>>,
    modes: Option<Vec<Mode>>,
}

#[derive(Serialize, Deserialize)]
pub struct Parameter {
    pub identifier: String,
    pub name: String,
    pub type_of: String,
    pub required: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Mode {
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
}

pub fn enabled_mode(name: String, description: Option<String>) -> Result<Mode, anyhow::Error> {
    Ok(Mode {
        identifier: "enabled".to_string(),
        name,
        description,
    })
}

pub fn disabled_mode(name: String, description: Option<String>) -> Result<Mode, anyhow::Error> {
    Ok(Mode {
        identifier: "disabled".to_string(),
        name,
        description,
    })
}

pub struct About {}

impl About {
    pub async fn get_base_url() -> Result<String, anyhow::Error> {
        let mut res: spin_sdk::http::Response = spin_sdk::http::send(
            spin_sdk::http::Request::builder()
                .method(spin_sdk::http::Method::Get)
                .uri("http://hubro-release-api-svc.hubro.svc.cluster.local/plugins/base")
                .body(()).build(),
        )
        .await?;
        let body = std::str::from_utf8(res.body()).unwrap();
        return Ok(body.to_string());
    }

    pub async fn get_redirect_url(plugin_id: &str) -> Result<String, anyhow::Error> {
        let mut res: spin_sdk::http::Response = spin_sdk::http::send(
            spin_sdk::http::Request::builder()
                .method(spin_sdk::http::Method::Get)
                .uri(format!(
                    "http://hubro-release-api-svc.hubro.svc.cluster.local/plugins/redirect?pluginId={plugin_id}"
                ))
                .body(()).build(),
        )
        .await?;
        let body = std::str::from_utf8(res.body()).unwrap();
        return Ok(body.to_string());
    }

    pub async fn generate_info(
        identifier: &str,
        name: &str,
        short_description: &str,
        url: &str,
        version: &str,
        icon_url: &str,
        external_endpoints: Option<Vec<String>>,
        parameters: Option<Vec<Parameter>>,
        modes: Option<Vec<Mode>>,
    ) -> anyhow::Result<impl IntoResponse> {
        let plugin_info = PluginInfo {
            identifier: identifier.to_string(),
            name: name.to_string(),
            short_description: short_description.to_string(),
            url: url.to_string(),
            version: version.to_string(),
            icon_url: icon_url.to_string(),
            external_endpoints,
            parameters,
            modes,
        };
        let j = serde_json::to_string(&plugin_info)?;

        Ok(spin_sdk::http::Response::builder()
            .status(200)
            .body(j).build())
    }
}
