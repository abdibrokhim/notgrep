use std::time::{Instant, Duration};

use actix_web::{
    get, 
    web, 
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};
use serde_json::json;

// Import from our library crate
use notgrep::{
    search_github_code, 
    SearchQuery, 
    get_github_token,
    fetch_download_url_from_contents_url,
    download_file,
    highlight_and_count,
};

/// This endpoint responds to `GET /search?q=some+term` 
/// by performing a GitHub code search within user "abdibrokhim".
#[get("/search")]
async fn search_endpoint(query: web::Query<SearchQuery>) -> impl Responder {
    let start_time = Instant::now();
    let token = get_github_token();

    // Step 1: fetch the top-level GH code search
    let search_json = match search_github_code(&query.q, &token).await {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to search GitHub code: {}", e)
            }));
        }
    };

    // Extract items and top-level data from the search response
    let items = match search_json["items"].as_array() {
        Some(arr) => arr,
        None => {
            return HttpResponse::Ok().json(json!({
                "avatar_url": null,
                "hits": {
                    "hits": [],
                    "time": format!("{:?}", start_time.elapsed()),
                },
                "total_matches": 0
            }));
        }
    };

    // Grab avatar_url from the first item (if any)
    let avatar_url = items
        .get(0)
        .and_then(|item| item["repository"]["owner"]["avatar_url"].as_str())
        .unwrap_or("")
        .to_string();

    let mut hits = vec![];
    let mut total_matches = 0;

    // Loop over each item from the code search
    for item in items {
        // 1) Build the "contents_url" by replacing "{+path}" with the item.path
        let contents_url_template = item["repository"]["contents_url"].as_str().unwrap_or("");
        let file_path = item["path"].as_str().unwrap_or("");
        let contents_url = contents_url_template.replace("{+path}", file_path);

        // 2) From that contents_url, fetch the JSON to extract "download_url"
        let download_url = match fetch_download_url_from_contents_url(&contents_url).await {
            Ok(opt) => opt,
            Err(_) => None,
        };

        // If there's no valid download_url, skip
        if download_url.is_none() {
            continue;
        }
        let download_url = download_url.unwrap();

        // 3) Download the raw file
        let file_contents = match download_file(&download_url).await {
            Ok(txt) => txt,
            Err(_) => String::new(),
        };

        // 4) Highlight & count occurrences
        let highlight_result = highlight_and_count(&file_contents, &query.q);

        // Add to total matches
        total_matches += highlight_result.total_matches;

        // Build a partial JSON for this "hit"
        let path_raw = item["path"].as_str().unwrap_or("").to_string();
        let repo_full_name = item["repository"]["full_name"].as_str().unwrap_or("").to_string();
        let hit_json = json!({
            "content": {
                "snippet": highlight_result.snippet,
                "full": highlight_result.full
            },
            "path": {
                "raw": path_raw
            },
            "repo": {
                "raw": repo_full_name
            },
            "total_matches": {
                "raw": highlight_result.total_matches
            }
        });

        hits.push(hit_json);
    }

    // Build the final response
    let elapsed = start_time.elapsed();
    let response_json = json!({
        "avatar_url": avatar_url,
        "hits": {
            "hits": hits,
            "time": format!("{:?}", elapsed)
        },
        "total_matches": total_matches
    });

    HttpResponse::Ok().json(response_json)
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Start HTTP server on localhost:8080
//     HttpServer::new(|| {
//         App::new()
//             .service(search_endpoint) // Register the /search endpoint
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // For testing - return static JSON with delay
    let json_content = tokio::fs::read_to_string("test_result_20250110_062909.json").await?;
    HttpServer::new(move || {
        let json_content = json_content.clone();
        App::new()
            .service(web::resource("/search").to(move || {
                let json = json_content.clone();
                async move {
                    tokio::time::sleep_until(tokio::time::Instant::now() + Duration::from_secs(2)).await;
                    HttpResponse::Ok().content_type("application/json").body(json)
                }
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
