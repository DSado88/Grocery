use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{GroceryError, GroceryResult};

/// Recipe scoring weights — must sum to 1.0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub ingredient_overlap: f64,
    pub protein_alignment: f64,
    pub cuisine_affinity: f64,
    pub practical_friction: f64,
    pub family_fit: f64,
}

/// Ingredient mapping entry from the scoring config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientMapping {
    #[serde(default)]
    pub model_item: Option<String>,
    pub tier: u8,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Flavor booster keywords grouped by impact level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorBoosters {
    #[serde(default)]
    pub high: Vec<String>,
    #[serde(default)]
    pub medium: Vec<String>,
}

/// Untapped ingredient opportunity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntappedOpportunity {
    pub ingredient: String,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub recipes_using: Option<u32>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Recipe source configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeSource {
    pub name: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub scrape_method: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub search_url: Option<String>,
    #[serde(default)]
    pub fallback: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Full scoring configuration parsed from recipe-scoring-config.yaml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringConfig {
    pub weights: ScoringWeights,
    #[serde(default)]
    pub protein_scores: HashMap<String, u32>,
    #[serde(default)]
    pub cuisine_scores: HashMap<String, u32>,
    #[serde(default)]
    pub ingredient_map: HashMap<String, IngredientMapping>,
    #[serde(default)]
    pub flavor_boosters: Option<FlavorBoosters>,
    #[serde(default)]
    pub untapped: Vec<UntappedOpportunity>,
    #[serde(default)]
    pub sources: Vec<RecipeSource>,
}

impl ScoringConfig {
    /// Load from a YAML file path.
    pub fn from_file(path: &Path) -> GroceryResult<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    /// Parse from a YAML string.
    pub fn from_yaml(yaml: &str) -> GroceryResult<Self> {
        serde_yaml::from_str(yaml)
            .map_err(|e| GroceryError::ScoringConfigParse(e.to_string()))
    }

    /// Look up the protein score (0-100) for a protein key.
    pub fn protein_score(&self, protein: &str) -> u32 {
        self.protein_scores
            .get(protein)
            .copied()
            .unwrap_or(0)
    }

    /// Look up the cuisine affinity score (0-100) for a cuisine key.
    pub fn cuisine_score(&self, cuisine: &str) -> u32 {
        self.cuisine_scores
            .get(cuisine)
            .copied()
            .unwrap_or_else(|| {
                self.cuisine_scores
                    .get("general")
                    .copied()
                    .unwrap_or(50)
            })
    }

    /// Resolve a recipe ingredient name to its tier (0-3).
    /// Returns `None` if the ingredient is not in the map.
    pub fn ingredient_tier(&self, ingredient: &str) -> Option<u8> {
        let lower = ingredient.to_lowercase();
        for mapping in self.ingredient_map.values() {
            if mapping.aliases.iter().any(|a| a.to_lowercase() == lower) {
                return Some(mapping.tier);
            }
        }
        None
    }
}

#[cfg(test)]
#[path = "scoring_tests.rs"]
mod tests;
