use std::str;
const USER_AGENT: &str = "spin-sdk-rust";
const SIDECAR_URL: &str = "http://hubro-release-sidecar-svc.hubro.svc.cluster.local";
const INTERCEPT_OTP_ENDPOINT: &str = "/otp";

pub struct Client {

}

impl Client {
    pub async fn intercept_email_otp(email: &str, password: &str, imap: &str) -> Result<String, ()> {

        for _ in 0..100 {
            let res: http::Response<Vec<u8>> = spin_sdk::http::send(
                http::Request::builder()
                    .method("GET")
                    .header("Content-Type", "application/json")
                    .header("User-Agent", USER_AGENT)
                    .uri(format!("{SIDECAR_URL}{INTERCEPT_OTP_ENDPOINT}"))
                    .body(()).unwrap(),
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