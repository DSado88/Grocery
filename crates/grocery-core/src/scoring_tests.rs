use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_parse_scoring_config_from_real_yaml() -> TestResult {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("recipe-scoring-config.yaml"))
        .ok_or("could not resolve recipe-scoring-config.yaml path")?;

    if !path.exists() {
        // Skip if running outside the workspace root
        return Ok(());
    }

    let config = ScoringConfig::from_file(&path)?;

    // Weights should sum to ~1.0
    let sum = config.weights.ingredient_overlap
        + config.weights.protein_alignment
        + config.weights.cuisine_affinity
        + config.weights.practical_friction
        + config.weights.family_fit;
    assert!((sum - 1.0).abs() < 0.01, "weights sum to {sum}, expected 1.0");

    // Protein scores should include ground_chicken at 100
    assert_eq!(config.protein_score("ground_chicken"), 100);

    // Cuisine scores should include southeast_asian at 90
    assert_eq!(config.cuisine_score("southeast_asian"), 90);

    // Ingredient map should have cucumber at tier 1
    assert_eq!(config.ingredient_tier("cucumber"), Some(1));

    Ok(())
}

#[test]
fn test_scoring_weights_parse() -> TestResult {
    let yaml = r#"
weights:
  ingredient_overlap: 0.40
  protein_alignment: 0.20
  cuisine_affinity: 0.15
  practical_friction: 0.15
  family_fit: 0.10
"#;
    let config = ScoringConfig::from_yaml(yaml)?;
    assert!((config.weights.ingredient_overlap - 0.40).abs() < f64::EPSILON);
    assert!((config.weights.protein_alignment - 0.20).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_protein_score_lookup() -> TestResult {
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
"#;
    let config = ScoringConfig::from_yaml(yaml)?;
    assert_eq!(config.protein_score("ground_chicken"), 100);
    assert_eq!(config.protein_score("tofu"), 80);
    assert_eq!(config.protein_score("unknown"), 0);
    Ok(())
}

#[test]
fn test_cuisine_score_falls_back_to_general() -> TestResult {
    let yaml = r#"
weights:
  ingredient_overlap: 0.40
  protein_alignment: 0.20
  cuisine_affinity: 0.15
  practical_friction: 0.15
  family_fit: 0.10
cuisine_scores:
  general: 50
  korean: 85
"#;
    let config = ScoringConfig::from_yaml(yaml)?;
    assert_eq!(config.cuisine_score("korean"), 85);
    assert_eq!(config.cuisine_score("nonexistent"), 50);
    Ok(())
}

#[test]
fn test_ingredient_tier_alias_lookup() -> TestResult {
    let yaml = r#"
weights:
  ingredient_overlap: 0.40
  protein_alignment: 0.20
  cuisine_affinity: 0.15
  practical_friction: 0.15
  family_fit: 0.10
ingredient_map:
  cucumber:
    model_item: "Hot House Cucumber"
    tier: 1
    aliases: ["cucumber", "english cucumber", "seedless cucumber"]
  garlic:
    tier: 3
    aliases: ["garlic", "garlic cloves", "minced garlic"]
"#;
    let config = ScoringConfig::from_yaml(yaml)?;
    assert_eq!(config.ingredient_tier("cucumber"), Some(1));
    assert_eq!(config.ingredient_tier("English Cucumber"), Some(1));
    assert_eq!(config.ingredient_tier("garlic cloves"), Some(3));
    assert_eq!(config.ingredient_tier("truffle oil"), None);
    Ok(())
}
