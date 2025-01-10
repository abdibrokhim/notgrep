use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

/// Example struct to hold query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

/// Example function that calls GitHub code search
/// Helper function to call the GitHub Code Search API restricted to `user:abdibrokhim`.
/// We specifically use the `/search/code` endpoint.
pub async fn search_github_code(query: &str, token: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // Construct the code search query strictly within user:abdibrokhim
    // E.g., "import React" => "import React+user:abdibrokhim"
    // URL-encoded automatically by reqwest's builder if we use .query() properly;
    // but let's manually do the query string for simplicity.
    let search_query = format!("{}+user:abdibrokhim", query);

    // GitHub Code Search endpoint
    // Full docs: https://docs.github.com/en/rest/search#search-code
    let url = format!("https://api.github.com/search/code?q={}", search_query);

    // Build HTTP client
    let client = Client::new();

    // Make the GET request to GitHub Search API
    let response = client
        .get(&url)
        // GitHub requires a User-Agent header
        .header("User-Agent", "Actix-web GitHub Code Search App")
        // If needed, add an Authorization header for private repos or higher rate limits:
        .header("Authorization", format!("token {}", token))
        .send()
        .await?;

    let json: Value = response.json().await?;  // Parse response to JSON Value
    Ok(json)
}