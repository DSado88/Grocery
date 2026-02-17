use thiserror::Error;

#[derive(Debug, Error)]
pub enum GroceryError {
    #[error("failed to parse household model: {0}")]
    HouseholdParse(String),

    #[error("failed to parse recipe data: {0}")]
    RecipeParse(String),

    #[error("failed to parse scoring config: {0}")]
    ScoringConfigParse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("item not found: {0}")]
    ItemNotFound(String),

    #[error("recipe not found: {0}")]
    RecipeNotFound(String),
}

pub type GroceryResult<T> = Result<T, GroceryError>;
