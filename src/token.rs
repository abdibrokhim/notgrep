use std::env;
use dotenv::dotenv;

/// Load GitHub token from the .env file/environment variable.
/// Expects a key named "GITHUB_TOKEN" in .env or system env.
pub fn get_github_token() -> String {
    dotenv().ok(); // Initializes .env reading (ignore any load errors).
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable not set")
}