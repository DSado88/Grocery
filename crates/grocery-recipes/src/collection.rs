use std::path::Path;

use grocery_core::error::GroceryResult;
use grocery_core::recipe::Recipe;
use grocery_core::scoring::ScoringConfig;

use crate::matcher::{self, DEFAULT_THRESHOLD};
use crate::scorer::{self, RecipeScore};

/// A collection of recipes loaded from JSON.
#[derive(Debug, Clone)]
pub struct RecipeCollection {
    recipes: Vec<Recipe>,
}

impl RecipeCollection {
    /// Load from a JSON file on disk.
    pub fn from_json_file(path: &Path) -> GroceryResult<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json(&content)
    }

    /// Parse from a JSON string.
    pub fn from_json(json: &str) -> GroceryResult<Self> {
        let recipes: Vec<Recipe> = serde_json::from_str(json)?;
        Ok(Self { recipes })
    }

    /// Fuzzy-match recipes by name.
    ///
    /// Returns `(index, recipe, similarity)` sorted by similarity descending.
    pub fn find_by_name(&self, query: &str) -> Vec<(usize, &Recipe, f64)> {
        matcher::find_recipes_by_name(&self.recipes, query, DEFAULT_THRESHOLD)
            .into_iter()
            .filter_map(|m| {
                self.recipes.get(m.index).map(|r| (m.index, r, m.similarity))
            })
            .collect()
    }

    /// Return all recipes that have ingredient data.
    pub fn with_ingredients(&self) -> Vec<&Recipe> {
        self.recipes.iter().filter(|r| r.has_ingredients()).collect()
    }

    /// Filter recipes by primary protein (case-insensitive substring match).
    pub fn filter_by_protein(&self, protein: &str) -> Vec<&Recipe> {
        let lower = protein.to_lowercase();
        self.recipes
            .iter()
            .filter(|r| {
                r.primary_protein
                    .as_ref()
                    .is_some_and(|p| p.to_lowercase().contains(&lower))
            })
            .collect()
    }

    /// Filter recipes by tag (case-insensitive exact match on any tag).
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Recipe> {
        let lower = tag.to_lowercase();
        self.recipes
            .iter()
            .filter(|r| r.tags.iter().any(|t| t.to_lowercase() == lower))
            .collect()
    }

    /// Score every recipe that has ingredients.
    ///
    /// Returns `(index, RecipeScore)` sorted by score descending.
    pub fn score_all(&self, config: &ScoringConfig) -> Vec<(usize, RecipeScore)> {
        let mut scored: Vec<(usize, RecipeScore)> = self
            .recipes
            .iter()
            .enumerate()
            .filter(|(_, r)| r.has_ingredients())
            .map(|(i, r)| (i, scorer::score_recipe(r, config)))
            .collect();

        scored.sort_by(|a, b| {
            b.1.overall
                .partial_cmp(&a.1.overall)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        scored
    }

    /// Number of recipes in the collection.
    pub fn len(&self) -> usize {
        self.recipes.len()
    }

    /// Whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.recipes.is_empty()
    }

    /// Access the underlying recipe slice.
    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[cfg(test)]
#[path = "collection_tests.rs"]
mod tests;
