use grocery_core::recipe::Recipe;

/// Result of a fuzzy recipe name match.
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub index: usize,
    pub similarity: f64,
}

/// Default similarity threshold for fuzzy matching.
pub const DEFAULT_THRESHOLD: f64 = 0.7;

/// Find recipes matching a query by name (fuzzy + substring).
///
/// Returns results sorted by similarity descending.
pub fn find_recipes_by_name(
    recipes: &[Recipe],
    query: &str,
    threshold: f64,
) -> Vec<MatchResult> {
    let lower_query = query.to_lowercase();

    let mut results: Vec<MatchResult> = recipes
        .iter()
        .enumerate()
        .filter_map(|(index, recipe)| {
            let lower_name = recipe.name.to_lowercase();

            let similarity = if lower_name.contains(&lower_query) {
                1.0
            } else {
                strsim::jaro_winkler(&lower_query, &lower_name)
            };

            if similarity >= threshold {
                Some(MatchResult { index, similarity })
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| {
        b.similarity
            .partial_cmp(&a.similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    results
}

#[cfg(test)]
#[path = "matcher_tests.rs"]
mod tests;
