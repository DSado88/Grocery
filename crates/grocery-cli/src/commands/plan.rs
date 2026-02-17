use std::path::Path;

use grocery_core::household::HouseholdModel;
use grocery_core::scoring::ScoringConfig;
use grocery_list::{consolidate, generate_list, ShoppingList};
use grocery_recipes::RecipeCollection;

/// Output format for the shopping list.
#[derive(Clone, Debug, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Compact,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "compact" => Ok(Self::Compact),
            other => Err(format!("unknown format: {other} (expected text, json, or compact)")),
        }
    }
}

/// Run the `plan` subcommand.
pub fn run(
    data_dir: &Path,
    recipe_names: &[String],
    format: &OutputFormat,
    include_staples: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let household = HouseholdModel::from_file(&data_dir.join("household-model.yaml"))?;
    let config = ScoringConfig::from_file(&data_dir.join("recipe-scoring-config.yaml"))?;
    let collection = RecipeCollection::from_json_file(&data_dir.join("recipe-links.json"))?;

    // Resolve recipe names via fuzzy matching
    let mut matched_recipes = Vec::new();
    for name in recipe_names {
        let results = collection.find_by_name(name);
        if let Some((_, recipe, similarity)) = results.first() {
            eprintln!("  Matched \"{}\" -> \"{}\" ({:.0}%)", name, recipe.name, similarity * 100.0);
            matched_recipes.push(*recipe);
        } else {
            eprintln!("  Warning: no match found for \"{}\"", name);
        }
    }

    if matched_recipes.is_empty() && !include_staples {
        eprintln!("No recipes matched and staples disabled. Nothing to generate.");
        return Ok(());
    }

    // Generate the list
    let household_for_gen = if include_staples {
        &household
    } else {
        // Use an empty household to skip staples
        &HouseholdModel::from_yaml("family:\n  members: []\nstores: {}")?
    };

    let items = generate_list(&matched_recipes, household_for_gen, &config);
    let items = consolidate(items);
    let list = ShoppingList::new(items);

    if list.is_empty() {
        eprintln!("Shopping list is empty — selected recipes may not have ingredient data.");
        return Ok(());
    }

    // Output
    match format {
        OutputFormat::Text => print!("{}", list.format_text()),
        OutputFormat::Json => println!("{}", list.format_json()?),
        OutputFormat::Compact => print!("{}", list.format_compact()),
    }

    Ok(())
}
