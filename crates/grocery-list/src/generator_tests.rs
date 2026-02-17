use super::*;
use grocery_core::household::HouseholdModel;
use grocery_core::item::ItemSource;
use grocery_core::scoring::ScoringConfig;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn test_household() -> Result<HouseholdModel, Box<dyn std::error::Error>> {
    let yaml = r#"
family:
  members:
    - name: Test
stores: {}
giant_recurring:
  - item: "English Cucumber"
    category: produce
    frequency: "14/18"
  - item: "Cilantro Fresh"
    category: produce
    frequency: "13/18"
  - item: "Ground Chicken"
    category: meat
    frequency: "14/18"
  - item: "Rare Item"
    category: snacks
    frequency: "1/18"
"#;
    Ok(HouseholdModel::from_yaml(yaml)?)
}

fn test_config() -> Result<ScoringConfig, Box<dyn std::error::Error>> {
    let yaml = r#"
weights:
  ingredient_overlap: 0.40
  protein_alignment: 0.20
  cuisine_affinity: 0.15
  practical_friction: 0.15
  family_fit: 0.10
ingredient_map:
  cucumber:
    model_item: "English Cucumber"
    tier: 1
    aliases: ["cucumber", "english cucumber"]
  cilantro:
    model_item: "Cilantro Fresh"
    tier: 1
    aliases: ["cilantro", "fresh cilantro"]
  garlic:
    model_item: "Garlic"
    tier: 3
    aliases: ["garlic", "garlic cloves", "minced garlic"]
  ground_chicken:
    model_item: "Ground Chicken"
    tier: 1
    aliases: ["ground chicken", "chicken mince"]
"#;
    Ok(ScoringConfig::from_yaml(yaml)?)
}

fn test_recipe(json: &str) -> Result<Recipe, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(json)?)
}

// ── generate_list ───────────────────────────────────────────────

#[test]
fn test_generate_staples_only() -> TestResult {
    let household = test_household()?;
    let config = test_config()?;
    let recipes: Vec<&Recipe> = vec![];

    let items = generate_list(&recipes, &household, &config);

    // Only tier 1 (EveryOrder) items: cucumber (14/18), cilantro (13/18), ground chicken (14/18)
    // "Rare Item" (1/18) should NOT be included
    assert_eq!(items.len(), 3);
    assert!(items.iter().all(|i| i.source == ItemSource::Staple));
    Ok(())
}

#[test]
fn test_generate_recipes_only() -> TestResult {
    let yaml = r#"
family:
  members: []
stores: {}
"#;
    let household = HouseholdModel::from_yaml(yaml)?;
    let config = test_config()?;

    let recipe = test_recipe(r#"{
        "name": "Test Dish",
        "url": "https://example.com",
        "ingredients": ["2 english cucumber", "4 garlic cloves", "truffle oil"]
    }"#)?;
    let recipes = vec![&recipe];

    let items = generate_list(&recipes, &household, &config);

    assert_eq!(items.len(), 3);
    assert!(items.iter().all(|i| matches!(&i.source, ItemSource::Recipe(name) if name == "Test Dish")));
    Ok(())
}

#[test]
fn test_generate_recipes_and_staples() -> TestResult {
    let household = test_household()?;
    let config = test_config()?;

    let recipe = test_recipe(r#"{
        "name": "Garlic Dish",
        "url": "https://example.com",
        "ingredients": ["4 garlic cloves", "fresh cilantro"]
    }"#)?;
    let recipes = vec![&recipe];

    let items = generate_list(&recipes, &household, &config);

    // 3 staples + 2 recipe ingredients = 5 (not yet deduped)
    assert_eq!(items.len(), 5);

    let staple_count = items.iter().filter(|i| i.source == ItemSource::Staple).count();
    assert_eq!(staple_count, 3);

    let recipe_count = items
        .iter()
        .filter(|i| matches!(&i.source, ItemSource::Recipe(_)))
        .count();
    assert_eq!(recipe_count, 2);
    Ok(())
}

#[test]
fn test_generate_empty_inputs() -> TestResult {
    let yaml = r#"
family:
  members: []
stores: {}
"#;
    let household = HouseholdModel::from_yaml(yaml)?;
    let config = test_config()?;
    let recipes: Vec<&Recipe> = vec![];

    let items = generate_list(&recipes, &household, &config);
    assert!(items.is_empty());
    Ok(())
}

#[test]
fn test_resolve_ingredient_known() -> TestResult {
    let config = test_config()?;
    let item = resolve_ingredient("4 garlic cloves, minced", "Test", &config);
    assert_eq!(item.name, "Garlic");
    assert!(matches!(item.source, ItemSource::Recipe(ref name) if name == "Test"));
    Ok(())
}

#[test]
fn test_resolve_ingredient_unknown() -> TestResult {
    let config = test_config()?;
    let item = resolve_ingredient("2 Tbsp. fish sauce", "Test", &config);
    // Should fall back to cleaned raw string
    assert_eq!(item.name, "fish sauce");
    Ok(())
}

#[test]
fn test_resolve_ingredient_uses_model_item_name() -> TestResult {
    let config = test_config()?;
    let item = resolve_ingredient("fresh cilantro", "Test", &config);
    assert_eq!(item.name, "Cilantro Fresh");
    Ok(())
}

// ── clean_ingredient_name ───────────────────────────────────────

#[test]
fn test_clean_strips_quantity_and_unit() -> TestResult {
    assert_eq!(clean_ingredient_name("2 Tbsp. olive oil"), "olive oil");
    assert_eq!(clean_ingredient_name("1/2 cup coconut milk"), "coconut milk");
    assert_eq!(clean_ingredient_name("8 garlic cloves"), "garlic cloves");
    assert_eq!(clean_ingredient_name("1 lemon"), "lemon");
    Ok(())
}

#[test]
fn test_clean_no_quantity() -> TestResult {
    assert_eq!(clean_ingredient_name("salt and pepper"), "salt and pepper");
    assert_eq!(clean_ingredient_name("cilantro"), "cilantro");
    Ok(())
}

#[test]
fn test_clean_empty_string() -> TestResult {
    assert_eq!(clean_ingredient_name(""), "");
    assert_eq!(clean_ingredient_name("  "), "");
    Ok(())
}
