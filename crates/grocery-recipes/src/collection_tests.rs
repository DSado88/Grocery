use std::path::Path;

use super::*;
use grocery_core::scoring::ScoringConfig;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn small_collection_json() -> &'static str {
    r#"[
        {
            "name": "Sambal Noodles",
            "url": "https://example.com",
            "tags": ["Pork"],
            "primary_protein": "pork",
            "ingredients": ["sambal oelek", "ground pork", "soy sauce"]
        },
        {
            "name": "Tofu Stir Fry",
            "url": "https://example.com",
            "tags": ["Tofu", "Quick"],
            "primary_protein": "tofu",
            "ingredients": ["firm tofu", "garlic", "soy sauce"]
        },
        {
            "name": "Family Chicken",
            "url": "https://example.com",
            "tags": ["Easy"],
            "primary_protein": "chicken"
        }
    ]"#
}

fn test_config() -> Result<ScoringConfig, Box<dyn std::error::Error>> {
    let yaml = r#"
weights:
  ingredient_overlap: 0.40
  protein_alignment: 0.20
  cuisine_affinity: 0.15
  practical_friction: 0.15
  family_fit: 0.10
protein_scores:
  pork: 15
  tofu: 80
cuisine_scores:
  general: 50
ingredient_map:
  garlic:
    tier: 3
    aliases: ["garlic"]
  tofu:
    tier: 2
    aliases: ["tofu", "firm tofu"]
flavor_boosters:
  high: ["soy sauce", "sambal"]
  medium: []
"#;
    Ok(ScoringConfig::from_yaml(yaml)?)
}

#[test]
fn test_from_json_valid_array() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    assert_eq!(coll.len(), 3);
    assert!(!coll.is_empty());
    Ok(())
}

#[test]
fn test_from_json_empty_array() -> TestResult {
    let coll = RecipeCollection::from_json("[]")?;
    assert!(coll.is_empty());
    assert_eq!(coll.len(), 0);
    Ok(())
}

#[test]
fn test_from_json_invalid_returns_error() -> TestResult {
    let result = RecipeCollection::from_json("not json");
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_from_json_file_real_data() -> TestResult {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("recipe-links.json"))
        .ok_or("could not resolve recipe-links.json path")?;

    if !path.exists() {
        return Ok(());
    }

    let coll = RecipeCollection::from_json_file(&path)?;
    assert!(coll.len() >= 90, "expected ~91 recipes, got {}", coll.len());
    Ok(())
}

#[test]
fn test_with_ingredients_filters_correctly() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let with = coll.with_ingredients();
    // 2 of 3 recipes have ingredients
    assert_eq!(with.len(), 2);
    Ok(())
}

#[test]
fn test_filter_by_protein_case_insensitive() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let tofu = coll.filter_by_protein("Tofu");
    assert_eq!(tofu.len(), 1);
    assert_eq!(tofu[0].name, "Tofu Stir Fry");
    Ok(())
}

#[test]
fn test_filter_by_protein_no_match() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let lamb = coll.filter_by_protein("lamb");
    assert!(lamb.is_empty());
    Ok(())
}

#[test]
fn test_filter_by_tag_case_insensitive() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let pork = coll.filter_by_tag("pork");
    assert_eq!(pork.len(), 1);
    assert_eq!(pork[0].name, "Sambal Noodles");
    Ok(())
}

#[test]
fn test_filter_by_tag_no_match() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let result = coll.filter_by_tag("Nonexistent");
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_find_by_name_integration() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let results = coll.find_by_name("tofu");
    assert!(!results.is_empty());
    assert_eq!(results[0].1.name, "Tofu Stir Fry");
    Ok(())
}

#[test]
fn test_score_all_returns_sorted_descending() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let config = test_config()?;
    let scored = coll.score_all(&config);
    // Only recipes with ingredients get scored
    assert_eq!(scored.len(), 2);
    // Sorted descending
    assert!(scored[0].1.overall >= scored[1].1.overall);
    Ok(())
}

#[test]
fn test_score_all_skips_recipes_without_ingredients() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let config = test_config()?;
    let scored = coll.score_all(&config);
    // "Family Chicken" has no ingredients, should be excluded
    let indices: Vec<usize> = scored.iter().map(|(i, _)| *i).collect();
    assert!(!indices.contains(&2), "recipe index 2 (no ingredients) should be excluded");
    Ok(())
}

#[test]
fn test_recipes_accessor() -> TestResult {
    let coll = RecipeCollection::from_json(small_collection_json())?;
    let slice = coll.recipes();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0].name, "Sambal Noodles");
    Ok(())
}
