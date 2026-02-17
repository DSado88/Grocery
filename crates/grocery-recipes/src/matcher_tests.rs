use super::*;
use grocery_core::recipe::Recipe;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn test_recipes() -> Result<Vec<Recipe>, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(r#"[
        {"name": "Spicy-Sweet Sambal Pork Noodles", "url": "https://example.com"},
        {"name": "Cold Soba Noodles", "url": "https://example.com"},
        {"name": "Greek Chicken Meatballs", "url": "https://example.com"},
        {"name": "Sweet Potato Hash with Tofu", "url": "https://example.com"},
        {"name": "Big Ol Mess", "url": "https://example.com"}
    ]"#)?)
}

#[test]
fn test_find_exact_substring_match() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "Soba", DEFAULT_THRESHOLD);
    assert!(!results.is_empty());
    assert_eq!(results[0].index, 1);
    assert!((results[0].similarity - 1.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_find_case_insensitive() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "big ol mess", DEFAULT_THRESHOLD);
    assert!(!results.is_empty());
    assert_eq!(results[0].index, 4);
    Ok(())
}

#[test]
fn test_find_no_match_below_threshold() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "xyzzy", DEFAULT_THRESHOLD);
    assert!(results.is_empty());
    Ok(())
}

#[test]
fn test_find_results_sorted_by_similarity() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "Noodles", DEFAULT_THRESHOLD);
    // Both noodle recipes should match (substring)
    assert!(results.len() >= 2);
    // All results should be in descending similarity order
    for window in results.windows(2) {
        assert!(window[0].similarity >= window[1].similarity);
    }
    Ok(())
}

#[test]
fn test_find_threshold_filtering() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "chicken", 0.9);
    // Only exact substring matches (similarity 1.0) should pass 0.9 threshold
    for result in &results {
        assert!(result.similarity >= 0.9);
    }
    Ok(())
}

#[test]
fn test_find_empty_recipes_list() -> TestResult {
    let recipes: Vec<Recipe> = vec![];
    let results = find_recipes_by_name(&recipes, "anything", DEFAULT_THRESHOLD);
    assert!(results.is_empty());
    Ok(())
}

#[test]
fn test_find_partial_name_match() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "Potato", DEFAULT_THRESHOLD);
    assert!(!results.is_empty());
    assert_eq!(results[0].index, 3); // Sweet Potato Hash with Tofu
    Ok(())
}

#[test]
fn test_find_fuzzy_match() -> TestResult {
    let recipes = test_recipes()?;
    let results = find_recipes_by_name(&recipes, "greek chicken", 0.6);
    assert!(!results.is_empty());
    // Should include Greek Chicken Meatballs
    let has_greek = results.iter().any(|r| r.index == 2);
    assert!(has_greek, "expected Greek Chicken Meatballs in results");
    Ok(())
}
