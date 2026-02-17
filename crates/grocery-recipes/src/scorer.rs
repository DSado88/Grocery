use grocery_core::recipe::Recipe;
use grocery_core::scoring::ScoringConfig;

/// Per-dimension breakdown of a recipe score.
#[derive(Debug, Clone)]
pub struct DimensionScore {
    pub name: &'static str,
    pub raw_score: f64,
    pub weight: f64,
    pub weighted: f64,
}

/// Full recipe score with overall and per-dimension breakdowns.
#[derive(Debug, Clone)]
pub struct RecipeScore {
    pub overall: f64,
    pub label: &'static str,
    pub dimensions: Vec<DimensionScore>,
}

/// Score a recipe against the household scoring config.
///
/// Returns overall score (0-100) with per-dimension breakdowns.
pub fn score_recipe(recipe: &Recipe, config: &ScoringConfig) -> RecipeScore {
    let ingredient = score_ingredient_overlap(recipe, config);
    let protein = score_protein_alignment(recipe, config);
    let cuisine = score_cuisine_affinity(recipe, config);
    let friction = score_practical_friction(recipe, config);
    let family = score_family_fit(recipe);

    let dimensions = vec![
        make_dimension("Ingredient Overlap", ingredient, config.weights.ingredient_overlap),
        make_dimension("Protein Alignment", protein, config.weights.protein_alignment),
        make_dimension("Cuisine Affinity", cuisine, config.weights.cuisine_affinity),
        make_dimension("Practical Friction", friction, config.weights.practical_friction),
        make_dimension("Family Fit", family, config.weights.family_fit),
    ];

    let overall = dimensions
        .iter()
        .map(|d| d.weighted)
        .sum::<f64>()
        .clamp(0.0, 100.0);

    RecipeScore {
        overall,
        label: score_label(overall),
        dimensions,
    }
}

/// Map a numeric score to a human-readable label.
pub fn score_label(score: f64) -> &'static str {
    if score >= 80.0 {
        "Perfect fit"
    } else if score >= 60.0 {
        "Good fit"
    } else if score >= 40.0 {
        "Moderate fit"
    } else if score >= 20.0 {
        "Stretch"
    } else {
        "Adventure"
    }
}

fn make_dimension(name: &'static str, raw_score: f64, weight: f64) -> DimensionScore {
    DimensionScore {
        name,
        raw_score,
        weight,
        weighted: raw_score * weight,
    }
}

// ── Dimension 1: Ingredient Overlap ─────────────────────────────

fn score_ingredient_overlap(recipe: &Recipe, config: &ScoringConfig) -> f64 {
    if recipe.ingredients.is_empty() {
        return 0.0;
    }

    let mut points = 0.0;
    for ingredient in &recipe.ingredients {
        points += match resolve_ingredient_tier(ingredient, config) {
            Some(1) => 3.0,
            Some(2) => 2.0,
            Some(3) => 1.0,
            Some(0) => 0.5,
            _ => 0.0,
        };
    }

    let max_possible = recipe.ingredients.len() as f64 * 3.0;
    let mut score = (points / max_possible) * 100.0;

    // Apply flavor booster bonuses
    if let Some(ref boosters) = config.flavor_boosters {
        for ingredient in &recipe.ingredients {
            let lower = ingredient.to_lowercase();
            for keyword in &boosters.high {
                if lower.contains(&keyword.to_lowercase()) {
                    score += 10.0;
                }
            }
            for keyword in &boosters.medium {
                if lower.contains(&keyword.to_lowercase()) {
                    score += 5.0;
                }
            }
        }
    }

    score.clamp(0.0, 100.0)
}

/// Substring-match a recipe ingredient string against all aliases in the config.
///
/// Recipe ingredients are full descriptions like "8 garlic cloves, thinly sliced".
/// Returns the highest tier among all matching aliases.
fn resolve_ingredient_tier(ingredient: &str, config: &ScoringConfig) -> Option<u8> {
    let lower = ingredient.to_lowercase();
    let mut best_tier: Option<u8> = None;

    for mapping in config.ingredient_map.values() {
        for alias in &mapping.aliases {
            if lower.contains(&alias.to_lowercase()) {
                match best_tier {
                    None => best_tier = Some(mapping.tier),
                    Some(current) if mapping.tier > current => {
                        best_tier = Some(mapping.tier);
                    }
                    _ => {}
                }
            }
        }
    }

    best_tier
}

// ── Dimension 2: Protein Alignment ──────────────────────────────

fn score_protein_alignment(recipe: &Recipe, config: &ScoringConfig) -> f64 {
    let Some(ref protein) = recipe.primary_protein else {
        return 50.0;
    };

    let normalized = protein.to_lowercase().replace(' ', "_");

    // Exact lookup first
    let exact = config.protein_score(&normalized);
    if exact > 0 {
        return f64::from(exact);
    }

    // Partial match: check if either string contains the other
    for (key, &value) in &config.protein_scores {
        if normalized.contains(key) || key.contains(&normalized) {
            return f64::from(value);
        }
    }

    50.0
}

// ── Dimension 3: Cuisine Affinity ───────────────────────────────

fn score_cuisine_affinity(recipe: &Recipe, config: &ScoringConfig) -> f64 {
    let mut best: Option<u32> = None;

    for tag in &recipe.tags {
        let normalized = tag.to_lowercase().replace(' ', "_");

        if let Some(&score) = config.cuisine_scores.get(&normalized) {
            let current = best.unwrap_or(0);
            if score > current {
                best = Some(score);
            }
        }

        // Partial match: tag contains a cuisine key or vice versa
        for (key, &score) in &config.cuisine_scores {
            if normalized.contains(key) || key.contains(&normalized) {
                let current = best.unwrap_or(0);
                if score > current {
                    best = Some(score);
                }
            }
        }
    }

    f64::from(best.unwrap_or_else(|| config.cuisine_score("general")))
}

// ── Dimension 4: Practical Friction ─────────────────────────────

fn score_practical_friction(recipe: &Recipe, config: &ScoringConfig) -> f64 {
    let mut score: f64 = 80.0;

    for ingredient in &recipe.ingredients {
        match resolve_ingredient_tier(ingredient, config) {
            None => score -= 5.0,
            Some(0) => score -= 10.0,
            _ => {}
        }
    }

    score.clamp(0.0, 100.0)
}

// ── Dimension 5: Family Fit ─────────────────────────────────────

fn score_family_fit(recipe: &Recipe) -> f64 {
    let mut score: f64 = 60.0;

    // Servings bonus
    if let Some(ref s) = recipe.servings {
        if let Some(n) = extract_first_number(s) {
            if n >= 4 {
                score += 20.0;
            }
        }
    }

    // Cook time bonus (only if minutes, not hours)
    if let Some(ref ct) = recipe.cook_time {
        if !ct.is_empty() {
            let lower = ct.to_lowercase();
            if !lower.contains("hour") {
                if let Some(minutes) = extract_first_number(ct) {
                    if minutes <= 30 {
                        score += 10.0;
                    }
                }
            }
        }
    }

    // Tag-based bonus
    let has_easy_tag = recipe
        .tags
        .iter()
        .any(|t| {
            let lower = t.to_lowercase();
            lower == "easy" || lower == "quick"
        });
    if has_easy_tag {
        score += 10.0;
    }

    score.clamp(0.0, 100.0)
}

/// Extract the first integer from a string. E.g., "6-8 servings" -> Some(6).
fn extract_first_number(s: &str) -> Option<u32> {
    let mut start = None;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            if start.is_none() {
                start = Some(i);
            }
        } else if start.is_some() {
            break;
        }
    }

    let start = start?;
    let end = s[start..]
        .find(|c: char| !c.is_ascii_digit())
        .map_or(s.len(), |offset| start + offset);

    s[start..end].parse().ok()
}

#[cfg(test)]
#[path = "scorer_tests.rs"]
mod tests;
