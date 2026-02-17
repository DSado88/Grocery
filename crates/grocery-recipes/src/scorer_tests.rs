use super::*;
use grocery_core::scoring::ScoringConfig;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn test_config() -> Result<ScoringConfig, Box<dyn std::error::Error>> {
    let yaml = r#"
weights:
  ingredient_overlap: 0.40
  protein_alignment: 0.20
  cuisine_affinity: 0.15
  practical_friction: 0.15
  family_fit: 0.10
protein_scores:
  ground_chicken: 100
  tofu: 80
  pork: 15
  sausage: 20
cuisine_scores:
  southeast_asian: 90
  korean: 85
  general: 50
ingredient_map:
  cucumber:
    tier: 1
    aliases: ["cucumber", "english cucumber"]
  limes:
    tier: 1
    aliases: ["lime", "limes"]
  cilantro:
    tier: 1
    aliases: ["cilantro", "fresh cilantro"]
  garlic:
    tier: 3
    aliases: ["garlic", "garlic cloves"]
  tofu:
    tier: 2
    aliases: ["tofu", "firm tofu"]
  sausage:
    tier: 0
    aliases: ["sausage", "italian sausage"]
  ginger:
    tier: 0
    aliases: ["ginger", "fresh ginger"]
flavor_boosters:
  high: ["sambal", "gochujang", "cilantro", "lime", "soy sauce", "sesame"]
  medium: ["sriracha", "curry", "coconut", "avocado", "goat cheese"]
"#;
    Ok(ScoringConfig::from_yaml(yaml)?)
}

fn make_recipe(json: &str) -> Result<Recipe, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(json)?)
}

// ── Ingredient Overlap ──────────────────────────────────────────

#[test]
fn test_score_ingredient_overlap_all_tier1() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["cucumber", "limes", "cilantro"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let overlap = &score.dimensions[0];
    // 3 tier-1 ingredients: (3*3.0)/(3*3.0)*100 = 100, plus booster for cilantro+lime
    assert!(overlap.raw_score >= 100.0 - f64::EPSILON, "expected ~100, got {}", overlap.raw_score);
    Ok(())
}

#[test]
fn test_score_ingredient_overlap_mixed_tiers() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["cucumber", "garlic cloves", "firm tofu"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let overlap = &score.dimensions[0];
    // tier1(3.0) + tier3(1.0) + tier2(2.0) = 6.0 / 9.0 * 100 = 66.67
    assert!(overlap.raw_score > 60.0 && overlap.raw_score < 75.0,
        "expected ~67, got {}", overlap.raw_score);
    Ok(())
}

#[test]
fn test_score_ingredient_overlap_no_matches() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["truffle oil", "saffron", "wagyu"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let overlap = &score.dimensions[0];
    assert!(overlap.raw_score < f64::EPSILON, "expected 0, got {}", overlap.raw_score);
    Ok(())
}

#[test]
fn test_score_ingredient_overlap_empty_ingredients() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let overlap = &score.dimensions[0];
    assert!(overlap.raw_score < f64::EPSILON, "expected 0, got {}", overlap.raw_score);
    Ok(())
}

#[test]
fn test_score_ingredient_overlap_flavor_boosters() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["fresh cilantro", "lime juice", "garlic"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let overlap = &score.dimensions[0];
    // Base: (3+1+3)/(3*3)*100 = 77.8, then cilantro+lime boosters (+10 each from high keywords)
    // cilantro matches "cilantro" in high, lime juice matches "lime" in high
    assert!(overlap.raw_score > 77.0, "expected boosted score, got {}", overlap.raw_score);
    Ok(())
}

#[test]
fn test_score_ingredient_overlap_capped_at_100() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["cilantro", "limes", "cucumber", "sambal oelek", "soy sauce", "sesame oil"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let overlap = &score.dimensions[0];
    assert!(overlap.raw_score <= 100.0, "score should be capped at 100, got {}", overlap.raw_score);
    Ok(())
}

// ── Protein Alignment ───────────────────────────────────────────

#[test]
fn test_score_protein_known() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "primary_protein": "ground_chicken"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let protein = &score.dimensions[1];
    assert!((protein.raw_score - 100.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_protein_no_protein_defaults_to_50() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let protein = &score.dimensions[1];
    assert!((protein.raw_score - 50.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_protein_unknown_defaults_to_50() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "primary_protein": "ostrich"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let protein = &score.dimensions[1];
    assert!((protein.raw_score - 50.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_protein_partial_match() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "primary_protein": "sweet Italian sausage"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let protein = &score.dimensions[1];
    assert!((protein.raw_score - 20.0).abs() < f64::EPSILON,
        "expected 20 (sausage), got {}", protein.raw_score);
    Ok(())
}

// ── Cuisine Affinity ────────────────────────────────────────────

#[test]
fn test_score_cuisine_matching_tag() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "tags": ["Korean"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let cuisine = &score.dimensions[2];
    assert!((cuisine.raw_score - 85.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_cuisine_no_match_falls_back_to_general() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "tags": ["Grill", "Summer"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let cuisine = &score.dimensions[2];
    assert!((cuisine.raw_score - 50.0).abs() < f64::EPSILON,
        "expected 50 (general), got {}", cuisine.raw_score);
    Ok(())
}

// ── Practical Friction ──────────────────────────────────────────

#[test]
fn test_score_friction_all_known() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["cucumber", "garlic", "limes"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let friction = &score.dimensions[3];
    // All in map, no deductions: 80
    assert!((friction.raw_score - 80.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_friction_unknown_ingredients() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["truffle oil", "saffron", "wagyu"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let friction = &score.dimensions[3];
    // 80 - 3*5 = 65
    assert!((friction.raw_score - 65.0).abs() < f64::EPSILON,
        "expected 65, got {}", friction.raw_score);
    Ok(())
}

#[test]
fn test_score_friction_tier0_ingredients() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "ingredients": ["italian sausage", "fresh ginger"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let friction = &score.dimensions[3];
    // 80 - 2*10 = 60
    assert!((friction.raw_score - 60.0).abs() < f64::EPSILON,
        "expected 60, got {}", friction.raw_score);
    Ok(())
}

#[test]
fn test_score_friction_floor_at_zero() -> TestResult {
    let config = test_config()?;
    let ingredients: Vec<String> = (0..20).map(|i| format!("exotic_ingredient_{i}")).collect();
    let json = serde_json::json!({
        "name": "T", "url": "https://x.com",
        "ingredients": ingredients
    });
    let recipe: Recipe = serde_json::from_value(json)?;
    let score = score_recipe(&recipe, &config);
    let friction = &score.dimensions[3];
    assert!(friction.raw_score < f64::EPSILON, "expected 0, got {}", friction.raw_score);
    Ok(())
}

// ── Family Fit ──────────────────────────────────────────────────

#[test]
fn test_score_family_fit_large_servings_quick_cook() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "servings": "6-8 servings",
        "cook_time": "25 minutes"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let family = &score.dimensions[4];
    // 60 + 20 (servings >= 4) + 10 (cook_time <= 30) = 90
    assert!((family.raw_score - 90.0).abs() < f64::EPSILON,
        "expected 90, got {}", family.raw_score);
    Ok(())
}

#[test]
fn test_score_family_fit_no_info() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let family = &score.dimensions[4];
    // 60 base, no bonuses
    assert!((family.raw_score - 60.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_family_fit_hours_no_quick_bonus() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "cook_time": "3 hours 15 minutes"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let family = &score.dimensions[4];
    // 60 base, no cook time bonus (contains "hour")
    assert!((family.raw_score - 60.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_score_family_fit_easy_tag() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "tags": ["Easy", "Summer"]
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let family = &score.dimensions[4];
    // 60 + 10 (easy tag) = 70
    assert!((family.raw_score - 70.0).abs() < f64::EPSILON);
    Ok(())
}

// ── Overall + Labels ────────────────────────────────────────────

#[test]
fn test_score_overall_is_weighted_sum() -> TestResult {
    let config = test_config()?;
    let recipe = make_recipe(r#"{
        "name": "T", "url": "https://x.com",
        "tags": ["Korean"],
        "ingredients": ["cucumber", "garlic cloves", "firm tofu"],
        "primary_protein": "tofu",
        "servings": "4 servings",
        "cook_time": "20 min"
    }"#)?;
    let score = score_recipe(&recipe, &config);
    let expected: f64 = score.dimensions.iter().map(|d| d.weighted).sum();
    assert!((score.overall - expected).abs() < 0.01,
        "overall {} should equal weighted sum {}", score.overall, expected);
    Ok(())
}

#[test]
fn test_score_labels() -> TestResult {
    assert_eq!(score_label(95.0), "Perfect fit");
    assert_eq!(score_label(80.0), "Perfect fit");
    assert_eq!(score_label(79.9), "Good fit");
    assert_eq!(score_label(60.0), "Good fit");
    assert_eq!(score_label(59.9), "Moderate fit");
    assert_eq!(score_label(40.0), "Moderate fit");
    assert_eq!(score_label(39.9), "Stretch");
    assert_eq!(score_label(20.0), "Stretch");
    assert_eq!(score_label(19.9), "Adventure");
    assert_eq!(score_label(0.0), "Adventure");
    Ok(())
}

#[test]
fn test_extract_first_number_various() -> TestResult {
    assert_eq!(extract_first_number("6-8 servings"), Some(6));
    assert_eq!(extract_first_number("45 minutes"), Some(45));
    assert_eq!(extract_first_number("2 to 3 servings"), Some(2));
    assert_eq!(extract_first_number("no numbers here"), None);
    assert_eq!(extract_first_number(""), None);
    assert_eq!(extract_first_number("about 30min"), Some(30));
    Ok(())
}
