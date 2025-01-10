// ===========================================================
// 4) Count & highlight occurrences of exact phrase in the file
// ===========================================================

/// Returns:
/// - `total_matches`: total count of exact phrase matches
/// - `snippet`: the first three lines that contain the exact phrase, with highlighting
/// - `full`: the entire file with highlighting
pub struct HighlightResult {
    pub snippet: String,
    pub full: String,
    pub total_matches: usize
}

/// Highlights exact phrase matches by wrapping them in `<b>` tags.
pub fn highlight_text(original: &str, phrase: &str) -> (String, usize) {
    let mut highlighted = original.to_string();
    let count_total = count_occurrences(&highlighted, phrase);
    highlighted = highlighted.replace(
        phrase,
        &format!("<b>{}</b>", phrase)
    );
    (highlighted, count_total)
}

/// Count how many times exact phrase appears in text.
fn count_occurrences(text: &str, phrase: &str) -> usize {
    text.matches(phrase).count()
}

/// Build snippet: first three lines that contain the exact phrase
fn build_snippet(lines: &[&str], phrase: &str, max_lines: usize) -> String {
    let mut snippet_lines = vec![];
    for &line in lines {
        if line.contains(phrase) {
            snippet_lines.push(line);
        }
        if snippet_lines.len() == max_lines {
            break;
        }
    }
    snippet_lines.join("\n")
}

pub fn highlight_and_count(file_contents: &str, query: &str) -> HighlightResult {
    // Use exact query phrase without splitting
    let (highlighted_full, total_matches) = highlight_text(file_contents, query);

    // Build snippet with exact phrase matching
    let lines: Vec<&str> = file_contents.lines().collect();
    let snippet_raw = build_snippet(&lines, query, 3);
    let (snippet_highlighted, _) = highlight_text(&snippet_raw, query);

    HighlightResult {
        total_matches,
        snippet: snippet_highlighted,
        full: highlighted_full,
    }
}