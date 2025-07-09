
pub async fn fetch_ai() -> &str {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    return resp
}
