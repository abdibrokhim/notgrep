#[cfg(test)]
mod tests {
    use reqwest::Client;
    use std::fs::File;
    use std::io::Write;

    // If using async I/O in tests, use tokio's attribute macro:
    #[tokio::test]
    async fn test_search_import_react() -> Result<(), Box<dyn std::error::Error>> {
        // 1) Create an HTTP client
        let client = Client::new();

        // 2) Make a GET request to your local server’s "/search" endpoint
        //    with "import React" as the query
        let response_text = client
            .get("http://localhost:8080/search?q=linkedin.com/in/abdibrokhim")
            // If calling GitHub directly, remember to set User-Agent & possibly Authorization
            .header("User-Agent", "Actix-web GitHub Code Search Test")
            .send()
            .await?
            .text()
            .await?;

        // 3) Write the JSON response to a file named "test_result.json"
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("test_result_{}.json", timestamp);
        let mut file = File::create(filename)?;
        file.write_all(response_text.as_bytes())?;

        // Optionally, print to console for debugging
        println!("Response from server:\n{}", response_text);

        Ok(())
    }
}
