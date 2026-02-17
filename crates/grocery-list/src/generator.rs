use grocery_core::household::HouseholdModel;
use grocery_core::item::{ItemSource, ShoppingItem};
use grocery_core::recipe::Recipe;
use grocery_core::scoring::ScoringConfig;
use grocery_core::types::Category;

/// Generate a raw (not yet deduplicated) shopping list from recipes and household staples.
pub fn generate_list(
    recipes: &[&Recipe],
    household: &HouseholdModel,
    config: &ScoringConfig,
) -> Vec<ShoppingItem> {
    let mut items = Vec::new();

    // 1. Collect staples from household model (tier 1 Giant items)
    for staple in household.staples() {
        items.push(ShoppingItem {
            name: staple.item.clone(),
            quantity: 1,
            category: staple.category.clone(),
            source: ItemSource::Staple,
            note: None,
        });
    }

    // 2. Collect recipe ingredients
    for recipe in recipes {
        for ingredient in &recipe.ingredients {
            items.push(resolve_ingredient(ingredient, &recipe.name, config));
        }
    }

    items
}

/// Resolve a recipe ingredient string to a ShoppingItem.
///
/// Uses the scoring config's ingredient_map to find canonical names via substring
/// alias matching. Falls back to cleaning the raw ingredient string.
fn resolve_ingredient(
    ingredient: &str,
    recipe_name: &str,
    config: &ScoringConfig,
) -> ShoppingItem {
    let lower = ingredient.to_lowercase();

    // Try to match against ingredient_map aliases (substring match)
    let mut best_match: Option<(&str, &grocery_core::scoring::IngredientMapping)> = None;
    let mut best_alias_len = 0;

    for (key, mapping) in &config.ingredient_map {
        for alias in &mapping.aliases {
            let alias_lower = alias.to_lowercase();
            if lower.contains(&alias_lower) && alias_lower.len() > best_alias_len {
                best_match = Some((key.as_str(), mapping));
                best_alias_len = alias_lower.len();
            }
        }
    }

    if let Some((_key, mapping)) = best_match {
        let name = mapping
            .model_item
            .clone()
            .unwrap_or_else(|| clean_ingredient_name(ingredient));

        ShoppingItem {
            name,
            quantity: 1,
            category: Category::Other("mapped".to_string()),
            source: ItemSource::Recipe(recipe_name.to_string()),
            note: None,
        }
    } else {
        ShoppingItem {
            name: clean_ingredient_name(ingredient),
            quantity: 1,
            category: Category::Other("unknown".to_string()),
            source: ItemSource::Recipe(recipe_name.to_string()),
            note: None,
        }
    }
}

/// Strip leading quantity and unit text from a raw ingredient string.
///
/// Examples:
/// - "2 Tbsp. extra-virgin olive oil" → "extra-virgin olive oil"
/// - "1/2 cup coconut milk" → "coconut milk"
/// - "8 garlic cloves, thinly sliced" → "garlic cloves, thinly sliced"
/// - "salt and pepper" → "salt and pepper"
pub fn clean_ingredient_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let mut pos = 0;

    // Skip leading digits, fractions, decimals
    while pos < chars.len()
        && (chars[pos].is_ascii_digit() || chars[pos] == '/' || chars[pos] == '.')
    {
        pos += 1;
    }

    // If we consumed digits, skip whitespace after them
    if pos > 0 {
        while pos < chars.len() && chars[pos] == ' ' {
            pos += 1;
        }

        // Check if next word looks like a unit — skip it too
        let rest = &trimmed[pos..];
        let unit_end = skip_unit(rest);
        if unit_end > 0 {
            pos += unit_end;
            // Skip whitespace after unit
            while pos < chars.len() && chars[pos] == ' ' {
                pos += 1;
            }
        }
    }

    let result = trimmed[pos..].to_string();
    if result.is_empty() {
        trimmed.to_string()
    } else {
        result
    }
}

/// If the string starts with a common cooking unit, return how many bytes to skip.
fn skip_unit(s: &str) -> usize {
    let units = [
        "tablespoons", "tablespoon", "teaspoons", "teaspoon",
        "tbsp.", "tbsp", "tsp.", "tsp",
        "cups", "cup", "ounces", "ounce", "oz.",
        "pounds", "pound", "lbs.", "lbs", "lb.",
        "inches", "inch", "\"",
    ];

    let lower = s.to_lowercase();
    for unit in &units {
        if lower.starts_with(unit) {
            // Make sure the unit is followed by a space or end of string
            let after = &s[unit.len()..];
            if after.is_empty() || after.starts_with(' ') || after.starts_with('.') {
                // Include trailing period if present
                if after.starts_with('.') {
                    return unit.len() + 1;
                }
                return unit.len();
            }
        }
    }

    0
}

#[cfg(test)]
#[path = "generator_tests.rs"]
mod tests;
