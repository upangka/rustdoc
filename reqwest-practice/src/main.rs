use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut body = HashMap::new();
    body.insert("username", "pkmer");
    body.insert("email", "pkmer@example.com");

    let res = client
        .post("https://httpbin.org/post")
        .json(&body)
        .send()
        .await?;

    let body_text = res.text().await?;
    println!("响应文本:\n{body_text}");

    Ok(())
}
