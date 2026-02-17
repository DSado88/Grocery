use std::path::Path;

use grocery_core::scoring::ScoringConfig;
use grocery_recipes::{score_recipe, RecipeCollection};

/// Run the `score` subcommand.
pub fn run(
    data_dir: &Path,
    recipe_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = ScoringConfig::from_file(&data_dir.join("recipe-scoring-config.yaml"))?;
    let collection = RecipeCollection::from_json_file(&data_dir.join("recipe-links.json"))?;

    let results = collection.find_by_name(recipe_name);
    let (_, recipe, similarity) = results
        .first()
        .ok_or_else(|| format!("No recipe found matching \"{}\"", recipe_name))?;

    eprintln!("Matched \"{}\" -> \"{}\" ({:.0}%)\n", recipe_name, recipe.name, similarity * 100.0);

    if !recipe.has_ingredients() {
        eprintln!("Warning: this recipe has no ingredient data — scoring will be limited.\n");
    }

    let result = score_recipe(recipe, &config);

    println!("{}: {:.0}/100 — {}", recipe.name, result.overall, result.label);
    println!();

    for dim in &result.dimensions {
        println!(
            "  {:<22} {:.0}/100  (weight {:.0}%, contributes {:.1})",
            dim.name,
            dim.raw_score,
            dim.weight * 100.0,
            dim.weighted,
        );
    }

    println!();

    if let Some(ref protein) = recipe.primary_protein {
        println!("  Primary protein: {}", protein);
    }
    if let Some(ref servings) = recipe.servings {
        println!("  Servings: {}", servings);
    }
    if let Some(ref cook_time) = recipe.cook_time {
        if !cook_time.is_empty() {
            println!("  Cook time: {}", cook_time);
        }
    }

    Ok(())
}
