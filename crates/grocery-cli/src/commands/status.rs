use std::path::Path;

use grocery_core::household::HouseholdModel;
use grocery_core::types::FrequencyTier;
use grocery_recipes::RecipeCollection;

/// Run the `status` subcommand.
pub fn run(data_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let household = HouseholdModel::from_file(&data_dir.join("household-model.yaml"))?;
    let collection = RecipeCollection::from_json_file(&data_dir.join("recipe-links.json"))?;

    // Family
    println!("Family");
    for member in &household.family.members {
        let age = member
            .age
            .map(|a| format!(" (age {})", a))
            .unwrap_or_default();
        println!("  {}{}", member.name, age);
    }
    println!();

    // Giant recurring items by tier
    let tier1 = household.giant_items_by_tier(FrequencyTier::EveryOrder);
    let tier2 = household.giant_items_by_tier(FrequencyTier::MostOrders);
    let tier3 = household.giant_items_by_tier(FrequencyTier::Occasional);
    let rare = household.giant_items_by_tier(FrequencyTier::Rare);

    println!("Giant Recurring Items: {} total", household.giant_recurring.len());
    println!("  Every order (tier 1): {}", tier1.len());
    println!("  Most orders (tier 2): {}", tier2.len());
    println!("  Occasional  (tier 3): {}", tier3.len());
    println!("  Rare:                 {}", rare.len());
    println!();

    // Amazon recurring
    println!("Amazon Recurring Items: {}", household.amazon_recurring.len());
    println!();

    // Recipe collection stats
    let total = collection.len();
    let with_ingredients = collection.with_ingredients().len();
    println!("Recipe Collection: {} total ({} with ingredients)", total, with_ingredients);

    // Top proteins
    let mut protein_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for recipe in collection.recipes() {
        if let Some(ref p) = recipe.primary_protein {
            *protein_counts.entry(p.clone()).or_default() += 1;
        }
    }
    let mut proteins: Vec<_> = protein_counts.into_iter().collect();
    proteins.sort_by(|a, b| b.1.cmp(&a.1));

    if !proteins.is_empty() {
        println!("  Top proteins:");
        for (protein, count) in proteins.iter().take(5) {
            println!("    {}: {} recipes", protein, count);
        }
    }

    Ok(())
}
