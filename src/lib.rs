
use std::collections::HashMap;

#[tokio::main]
pub async fn fetch_ai() -> String {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await
        .json::<HashMap<String, String>>()
        .await;
    return resp
}
