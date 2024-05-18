use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::SecondsFormat::Millis;
use fhirbolt::model::r4b::resources::{Bundle, BundleEntry, ObservationEffective};
use fhirbolt::model::r4b::{Resource, types};
use fhirbolt::model::r4b::types::{Code, Coding, Period, Uri};
use serde_json::{json, Value};
use spin_sdk::http::Method::{Post};

pub struct Types {}

impl Types {
    pub const RECORD_TYPE_SMBG: &'static str = "14745-4";
    pub const RECORD_TYPE_STEPS: &'static str = "55423-8";
    pub const RECORD_TYPE_HDL: &'static str = "2085-9";
    pub const RECORD_TYPE_LDL: &'static str = "2089-1";
}

pub struct Client {
    pub base_url: Option<String>,
}

impl Client {

    const HUBRO_SERVICE_URL: &'static str = "http://hubro-api.hubro.svc.cluster.local";

    pub fn generate_bundle() -> anyhow::Result<Bundle> {
        Ok(Bundle {
            entry: vec![],
            r#type: types::Code {
                value: Some("Batch".into()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    pub fn generate_bundle_entry(record_type: &str, display: &str, start_time: &str, end_time: Option<&str>, unit: &str, value: &str, user_id: &str) -> anyhow::Result<BundleEntry> {
        let o = fhirbolt::model::r4b::resources::Observation {
            status: "final".into(),
            code: Box::new(types::CodeableConcept {
                coding: Vec::from([Coding {
                    code: Some(record_type.into()),
                    display: Some(display.into()),
                    system: Some("http://loinc.org".into()),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            subject: Some(Box::new(types::Reference {
                reference: Some(types::String::from(format!("Patient/{}", user_id).to_string())),
                ..Default::default()
            })),
            performer: Vec::from([types::Reference {
                reference: None,
                ..Default::default()
            }]),
            effective: match end_time {
                None => {
                    Some(ObservationEffective::DateTime(fhirbolt::model::r4b::types::DateTime {
                        value: Some(start_time.into()),
                        ..Default::default()
                    }))
                }
                Some(_) => {
                    Some(ObservationEffective::Period(Box::new(Period {
                        start: Some(fhirbolt::model::r4b::types::DateTime {
                            value: Some(start_time.into()),
                            ..Default::default()
                        }),
                        end: Some(fhirbolt::model::r4b::types::DateTime {
                            value: Some(end_time.unwrap().into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })))
                }
            },
            value: Some(fhirbolt::model::r4b::resources::ObservationValue::Quantity(Box::new(fhirbolt::model::r4b::types::Quantity {
                system: Some(Uri {
                    value: Some("http://unitsofmeasure.org".into()),
                    ..Default::default()
                }),
                code: Some(Code {
                    value: Some(unit.into()),
                    ..Default::default()
                }),
                value: Some(fhirbolt::model::r4b::types::Decimal {
                    value: Some(value.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        };
        let be = BundleEntry {
            id: None,
            extension: vec![],
            modifier_extension: vec![],
            link: vec![],
            full_url: None,
            resource: Some(Resource::Observation(Box::new(o))),
            search: None,
            request: None,
            response: None,
        };
        Ok(be)
    }

    pub async fn send_bundle(&self, bearer: &str, body: Option<Value>) -> Result<(), anyhow::Error> {
        let res: http::Response<Vec<u8>>;
        match self.base_url {
            None => {
                res = spin_sdk::http::send(
                    spin_sdk::http::Request::builder()
                        .method(Post)
                        .header("Accept", "application/json")
                        .header("Authorization", format!("Bearer {bearer}"))
                        .uri(format!("{}/plugins/post_data", Client::HUBRO_SERVICE_URL))
                        .body(Some(body.unwrap().to_string()))
                        .build(),
                ).await?;
            }
            Some(_) => {
                res = spin_sdk::http::send(
                    http::Request::builder()
                        .method("POST")
                        .header("Accept", "application/json")
                        .header("Authorization", format!("Bearer {bearer}"))
                        .header("Content-type", "application/json")
                        .uri(format!("{}/plugins/post_data", self.base_url.to_owned().unwrap()))
                        .body(Some(body.unwrap().to_string()))?
                ).await?;
            }
        }

        Ok(())
    }

    pub async fn anonymize_user(&self, user: &str, study_id: &str) -> Result<(String, String), anyhow::Error> {
        let data = json!({
          "user": user,
          "study": study_id
        });
        let res: http::Response<Vec<u8>> = spin_sdk::http::send(
            http::Request::builder()
                .method("POST")
                .header("Accept", "application/json")
                .header("Content-type", "application/json")
                .uri(format!("{}/plugins/anonymize", self.base_url.to_owned().unwrap()))
                .body(Some(data.to_string()))?
        ).await?;
        let body = std::str::from_utf8(res.body()).unwrap();
        let data: Value = serde_json::from_str(body)?;
        Ok((data["fhir_user"].as_str().unwrap().to_string(), data["fhir_token"].as_str().unwrap().to_string()))
    }
}