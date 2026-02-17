use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_parse_recipe_from_json() -> TestResult {
    let json = r#"{
        "name": "Spicy-Sweet Sambal Pork Noodles",
        "url": "https://www.bonappetit.com/recipe/spicy-sweet-sambal-pork-noodles",
        "tags": ["Pork"],
        "rating": 5,
        "needs_fixing": false,
        "last_made": null,
        "times_made": 0,
        "feedback": [],
        "source": "bon-appetit",
        "ingredients": [
            "2 Tbsp. extra-virgin olive oil",
            "2 lb. ground pork, divided",
            "1 2\" piece fresh ginger"
        ],
        "cook_time": "",
        "servings": "6–8 servings",
        "primary_protein": "pork"
    }"#;
    let recipe: Recipe = serde_json::from_str(json)?;
    assert_eq!(recipe.name, "Spicy-Sweet Sambal Pork Noodles");
    assert!(recipe.has_ingredients());
    assert_eq!(recipe.ingredients.len(), 3);
    assert_eq!(recipe.primary_protein.as_deref(), Some("pork"));
    Ok(())
}

#[test]
fn test_recipe_without_ingredients() -> TestResult {
    let json = r#"{
        "name": "Family Chicken Stir Fry",
        "url": "https://example.com",
        "tags": [],
        "rating": null,
        "times_made": 5
    }"#;
    let recipe: Recipe = serde_json::from_str(json)?;
    assert!(!recipe.has_ingredients());
    assert_eq!(recipe.times_made, 5);
    Ok(())
}

#[test]
fn test_parse_recipe_collection() -> TestResult {
    let json = r#"[
        {"name": "Recipe A", "url": "https://a.com"},
        {"name": "Recipe B", "url": "https://b.com", "ingredients": ["flour", "sugar"]}
    ]"#;
    let recipes: Vec<Recipe> = serde_json::from_str(json)?;
    assert_eq!(recipes.len(), 2);
    assert!(!recipes[0].has_ingredients());
    assert!(recipes[1].has_ingredients());
    Ok(())
}
