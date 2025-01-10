// Bring our module into scope
mod search;
mod token;
mod highlight;
mod download;

// Re-export the things you want to share
pub use search::{search_github_code, SearchQuery};
pub use token::get_github_token;
pub use highlight::{highlight_and_count, HighlightResult};
pub use download::{fetch_download_url_from_contents_url, download_file};