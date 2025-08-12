use jwt_compact::UntrustedToken;
use serde::{Deserialize, Serialize};
use std::str;
use spin_sdk::http::Method;
use spin_sdk::http::StatusCode;

const USER_AGENT: &str = "spin-sdk-rust";
// const SIDECAR_URL: &str = "http://127.0.0.1:8080";
const SIDECAR_URL: &str = "http://hubro-release-sidecar-svc.hubro.svc.cluster.local";
const INTERCEPT_OTP_ENDPOINT: &str = "/otp";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {}

pub struct Client {}

impl Client {
    pub fn verify_jwt(code: &str, offset: Option<i64>) -> Result<bool, ()> {
        let token = UntrustedToken::new(code).unwrap();
        let claims = token.deserialize_claims_unchecked::<Claims>().unwrap();
        let current_date_time =
            chrono::offset::Utc::now() + chrono::Duration::seconds(offset.unwrap_or(0));

        if claims.expiration.unwrap() <= current_date_time {
            return Ok(false);
        }
        return Ok(true);
    }

    pub async fn intercept_email_otp(
        email: &str,
        password: &str,
        imap: &str,
    ) -> Result<String, ()> {
        for _ in 0..100 {
            let res: spin_sdk::http::Response = spin_sdk::http::send(
                spin_sdk::http::Request::builder()
                    .method(Method::Get)
                    .header("User-Agent", USER_AGENT)
                    .uri(format!("{SIDECAR_URL}{INTERCEPT_OTP_ENDPOINT}?email={email}&password={password}&server={imap}"))
                    .body(()).build(),
            ).await.unwrap();
            let body = str::from_utf8(res.body()).unwrap();
            if !body.contains("undefined") {
                let token = body.replacen("\"", "", 2);
                return Ok(token);
            }
        }

        Err(())
    }
}
