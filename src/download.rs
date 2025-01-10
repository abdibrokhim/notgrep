use reqwest::Client;
use serde_json::Value;

// ==============================
// 2) Extract download_url
// ==============================
pub async fn fetch_download_url_from_contents_url(contents_url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .get(contents_url)
        .header("User-Agent", "Actix-web GitHub Code Search App")
        .send()
        .await?;

    if resp.status().is_success() {
        let json_val: Value = resp.json().await?;
        // "download_url" is typically a string in JSON
        if let Some(download_url) = json_val["download_url"].as_str() {
            Ok(Some(download_url.to_string()))
        } else {
            Ok(None)
        }
    } else {
        // Possibly 404 if file not found, or some other error
        Ok(None)
    }
}

// ===========================
// 3) Download raw file
// ===========================
pub async fn download_file(download_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .get(download_url)
        .header("User-Agent", "Actix-web GitHub Code Search App")
        .send()
        .await?;

    let text = resp.text().await?;
    Ok(text)
}