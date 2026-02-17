use serde::{Deserialize, Serialize};

/// A recipe from the collection (recipe-links.json schema).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub needs_fixing: bool,
    #[serde(default)]
    pub last_made: Option<String>,
    #[serde(default)]
    pub times_made: u32,
    #[serde(default)]
    pub feedback: Vec<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub ingredients: Vec<String>,
    #[serde(default)]
    pub cook_time: Option<String>,
    #[serde(default)]
    pub servings: Option<String>,
    #[serde(default)]
    pub primary_protein: Option<String>,
}

impl Recipe {
    /// Whether this recipe has ingredient data (needed for list generation).
    pub fn has_ingredients(&self) -> bool {
        !self.ingredients.is_empty()
    }
}

#[cfg(test)]
#[path = "recipe_tests.rs"]
mod tests;
